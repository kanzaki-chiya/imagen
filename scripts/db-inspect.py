import sqlite3
import os

path = os.path.join(
    os.environ["APPDATA"], "com.imagen.studio", "imagen.db"
)
c = sqlite3.connect(path)
tables = [
    row[0]
    for row in c.execute(
        "select name from sqlite_master where type='table' order by name"
    )
]
for table in tables:
    count = c.execute(f"select count(*) from {table}").fetchone()[0]
    print(f"{table}: {count} rows")
print("meta keys:", [r[0] for r in c.execute("select key from meta")])
print(
    "providers:",
    [r[:2] for r in c.execute("select id, name from providers")],
)
print(
    "presets:",
    [r[:2] for r in c.execute("select id, name from presets")],
)
