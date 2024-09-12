fn main() {
    let dbs = xhstt::xml::Archives::all_xml()
        .iter()
        .filter_map(|xml| {
            let xhstt = xhstt::parse(xml).instance().unwrap();

            match xhstt::db::Database::init(&xhstt) {
                Ok(x) => Some(x),
                Err(e) => {
                    dbg!(&xhstt.id);
                    println!("{:#?}", e);
                    None
                }
            }
        })
        .collect::<Vec<xhstt::db::Database>>();

    for db in dbs {
        let s = db.stats();
        println!(
            "{:<40}: assign_res = {:>8}, assign_class = {:>8}",
            db.instance_name,
            s.needs_resource_assignment(),
            s.needs_class_assignment()
        );
    }
}
