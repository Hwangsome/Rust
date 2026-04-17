// 这个文件演示 `Result<Option<T>, E>`：操作一定会执行，但结果可能是“找到值 / 没找到值 / 执行失败”三种之一。
// 运行时要观察：`None` 不是错误，它只是“这次查询合法完成，但没有数据”。
// 这类形状在数据库查询、缓存读取、配置查找里非常常见。
#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
}

#[derive(Debug)]
enum DbError {
    ConnectionFailed,
}

fn db_connection(available: bool) -> Result<(), DbError> {
    if available {
        Ok(())
    } else {
        Err(DbError::ConnectionFailed)
    }
}

fn find_product(id: u32, connection_available: bool) -> Result<Option<Product>, DbError> {
    db_connection(connection_available)?;

    match id {
        0..=100 => Ok(Some(Product {
            id,
            name: "Laptop".to_string(),
        })),
        _ => Ok(None),
    }
}

pub fn run() {
    println!("== Layered Outcomes: Result<Option<T>, E> ==");

    match find_product(10, true) {
        Ok(Some(product)) => println!(
            "existing product => id = {}, name = {}",
            product.id, product.name
        ),
        Ok(None) => println!("existing product => none"),
        Err(error) => println!("existing product => error: {:?}", error),
    }
    println!("missing product => {:?}", find_product(999, true));
    println!("connection failure => {:?}", find_product(10, false));
    println!();
}
