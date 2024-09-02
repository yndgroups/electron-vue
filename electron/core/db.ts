import Database from "better-sqlite3";
class Db {
  db: any;
  constructor() {
    this.db = new Database("/Users/mekings/system.db");
  }

  testQuery(userId: number) {
    const row = this.db.prepare("SELECT * FROM users WHERE id = ?").get(userId);
    console.log(row.firstName, row.lastName, row.email);
  }

  testCreateTable() {
    // db.exec(`DROP TABLE cats`)//删除表
    const create_table_cats = `CREATE TABLE cats
 (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name    CHAR(50)           NOT NULL,
    age   INT
 );`;

    this.db.exec(create_table_cats); //执行sql命令
  }
}

export default Db;
