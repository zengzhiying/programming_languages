#include<iostream>
#include<mysql-cppconn-8/jdbc/mysql_connection.h>
#include<mysql-cppconn-8/jdbc/mysql_driver.h>
#include<mysql-cppconn-8/jdbc/cppconn/exception.h>
#include<mysql-cppconn-8/jdbc/cppconn/statement.h>
#include<mysql-cppconn-8/jdbc/cppconn/resultset.h>

// driver download: https://dev.mysql.com/downloads/connector/cpp/
// g++ main.cpp -lmysqlcppconn -o main
// 当前 driver 版本: 8.4.0
int main() {
    sql::Driver *driver = get_driver_instance();
    sql::Connection *conn = driver->connect("tcp://192.168.10.80:9030", "root", "123456");
    conn->setSchema("test");
    sql::Statement *stmt =conn->createStatement();
    sql::ResultSet *res = stmt->executeQuery("SELECT id, data, name FROM test_cosine WHERE id in (5,6,7)");
    while(res->next()) {
        int rowId = res->getInt("id");
        std::string data = res->getString("data");
        std::string name = res->getString("name");
        std::cout << "id: " << rowId << " data: " << data << " name: " << name << std::endl;
    }
    res->close();
    stmt->close();
    conn->close();
    delete res;
    delete stmt;
    delete conn;
    
    return 0;
}
