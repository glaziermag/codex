import os
from typing import Dict, Iterator, Optional

try:
    import psycopg2
except ModuleNotFoundError:  # pragma: no cover - dependency may not be present
    psycopg2 = None


class PostgresStore:
    """Simple task storage using PostgreSQL."""

    def __init__(self, dsn: Optional[str] = None):
        if psycopg2 is None:
            raise RuntimeError("psycopg2 is required for PostgresStore")

        self.dsn = dsn or os.environ.get(
            "DATABASE_URL", "postgresql://user:password@localhost/tasks"
        )
        self.conn = psycopg2.connect(self.dsn)
        self.conn.autocommit = True
        self._ensure_tables()

    def _ensure_tables(self) -> None:
        with self.conn.cursor() as cur:
            cur.execute(
                """
                CREATE TABLE IF NOT EXISTS tasks (
                    id SERIAL PRIMARY KEY,
                    name TEXT NOT NULL
                )
                """
            )

    def add_task(self, name: str) -> Dict[str, str]:
        with self.conn.cursor() as cur:
            cur.execute(
                "INSERT INTO tasks(name) VALUES(%s) RETURNING id", (name,)
            )
            task_id = cur.fetchone()[0]
        return {"id": task_id, "name": name}

    def get_task(self, task_id: int) -> Optional[Dict[str, str]]:
        with self.conn.cursor() as cur:
            cur.execute("SELECT id, name FROM tasks WHERE id=%s", (task_id,))
            row = cur.fetchone()
        if row:
            return {"id": row[0], "name": row[1]}
        return None

    def list_tasks(self, limit: Optional[int] = None) -> Iterator[Dict[str, str]]:
        with self.conn.cursor() as cur:
            if limit:
                cur.execute(
                    "SELECT id, name FROM tasks ORDER BY id LIMIT %s", (limit,)
                )
            else:
                cur.execute("SELECT id, name FROM tasks ORDER BY id")
            for row in cur:
                yield {"id": row[0], "name": row[1]}
