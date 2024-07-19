use std::io::{Read, Write};
use tree_node::trans_vec_item_to_tree;
use xavier::from_xml;
use xml_parse::{H40002Object, H40003Object, H40003User, H40005Object, H40006Object, IOrg};

mod tree_node;
mod xml_parse;

fn main() -> anyhow::Result<()> {
    h40005()?;
    Ok(())
}

fn h40005() -> anyhow::Result<()> {
    let mut xml = String::new();
    let mut file = std::fs::File::open("h40005.xml")?;
    file.read_to_string(&mut xml)?;
    println!("{}", xml.len());
    let instance: H40005Object = from_xml(&xml)?;
    println!("{:?}", instance);
    Ok(())
}

fn h40003() -> anyhow::Result<()> {
    let mut xml = String::new();
    let mut file = std::fs::File::open("h40003.xml")?;
    file.read_to_string(&mut xml)?;
    println!("{}", xml.len());
    let instance: H40003Object = from_xml(&xml)?;
    let json: H40003User = serde_json::from_str(&instance.body.resp.mb.user_json)?;
    println!("{:?}", json);
    Ok(())
}

fn h40002() -> anyhow::Result<()> {
    let mut xml = String::new();
    let mut file = std::fs::File::open("h40002.xml")?;
    file.read_to_string(&mut xml)?;
    println!("{}", xml.len());
    let instance: H40002Object = from_xml(&xml)?;
    println!("{:?}", instance);
    Ok(())
}

fn h40006() -> anyhow::Result<()> {
    let mut xml = String::new();
    let mut file = std::fs::File::open("org.xml")?;
    file.read_to_string(&mut xml)?;
    println!("{}", xml.len());
    let instance: H40006Object = from_xml(&xml)?;
    // println!("{:?}",instance.body.resp);
    // let h40006_resp: H40006Resp = from_xml(&instance.body.resp)?;
    let orgs = instance
        .body
        .resp
        .orgs
        .into_iter()
        .map(TryInto::try_into)
        .collect::<anyhow::Result<Vec<IOrg>>>()?;
    let res = trans_vec_item_to_tree(orgs);

    let mut out = std::fs::File::create("org.json")?;
    let dd = serde_json::to_string(&res)?;
    out.write_all(dd.as_bytes())?;

    Ok(())
}
