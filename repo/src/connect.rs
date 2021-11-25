use rbatis::rbatis::Rbatis;

pub async fn connect_db() -> Rbatis {
    let rb = Rbatis::new();
    rb.link("mysql://bytom_dev:Bytom!Dev@rm-uf6k0307eqv8pb7hd1o.mysql.rds.aliyuncs.com:3306/blockcenter_bytom2.0").await.unwrap();
    rb
}
