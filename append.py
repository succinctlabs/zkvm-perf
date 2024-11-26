import gspread
import os
import json
import redis
import redis_lock
import csv, json, sys
from google.oauth2.service_account import Credentials

REDIS_HOST = os.getenv("REDIS_HOST")
REDIS_PASSWORD = os.getenv("REDIS_PASSWORD")
GSHEET_CLIENT_EMAIL = os.getenv("GSHEET_CLIENT_EMAIL")
GSHEET_PRIVATE_KEY = os.getenv("GSHEET_PRIVATE_KEY")

rc = redis.Redis(
    host=REDIS_HOST,
    port=6379,
    password=REDIS_PASSWORD,
    ssl=True
)

with redis_lock.Lock(rc, "zkvm-perf-wip-gh-action", expire=60):
    gc = gspread.service_account_from_dict({
        "type": "service_account",
        "project_id": "succinct-benchmarking",
        "client_email": GSHEET_CLIENT_EMAIL,
        "private_key": GSHEET_PRIVATE_KEY.replace("\\n", "\n"),
        "token_uri": "https://oauth2.googleapis.com/token",
    })
        
    sheet = gc.open("SP1 Datasheets")
    worksheet = sheet.worksheet("Runs")

    with open("./benchmarks/benchmarks_latest.csv") as f:
        reader = csv.reader(f)
        next(reader)
        data = list(reader)
        run_id = "${{ github.run_id }}"
        git_ref = "${{ github.sha }}"
        instance = "${{ matrix.instance }}"
        for row in data:
            row.insert(0, instance)
            row.insert(0, git_ref)
            row.insert(0, run_id)
            worksheet.append_row(row)
