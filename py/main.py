import yaml
import pandas as pd
import sys
import typing
import glob
import json


class Params:
    db: str
    until: typing.Optional[int] = None
    after: typing.Optional[int] = None
    price: typing.Optional[int] = None
    first: typing.Optional[int] = None
    last: typing.Optional[int] = None

    def __init__(self, db):
        self.db = db


def prepare_db(db_folder: str) -> pd.DataFrame:
    database_records = []
    files = glob.glob(str(db_folder + "/*.json"))

    for file in files:
        db = {}
        with open(file, "r") as db_file:
            db = json.loads(db_file.read())
            if db["mint_price"] == None:
                continue

        database_records.append(db)

    return pd.DataFrame().from_records(database_records)


if __name__ == "__main__":
    params_yml_path = sys.argv[1]
    params_yml = {}
    with open(params_yml_path, "r") as params_yml_file:
        params_yml = yaml.load(params_yml_file, Loader=yaml.Loader)

    query = []
    params = Params(params_yml["db"])
    database_df = prepare_db(params.db)
    database_df = database_df.sort_values(by="block_time", ascending=True).reset_index()

    if "until" in params_yml.keys():
        params.until = params_yml["until"]
        query.append(f"block_time < {params.until}")

    if "after" in params_yml.keys():
        params.after = params_yml["after"]
        query.append(f"block_time > {params.after}")

    if "price" in params_yml.keys():
        params.price = params_yml["price"]
        query.append(f"mint_price == {params.price}")

    if "first" in params_yml.keys():
        params.first = params_yml["first"]

    if "last" in params_yml.keys():
        params.last = params_yml["last"]

    report = database_df.query(" & ".join(query))

    if params.first != None:
        report = report[: params.first]

    if params.last != None:
        report = report[params.last :]

    print(report.to_csv())

