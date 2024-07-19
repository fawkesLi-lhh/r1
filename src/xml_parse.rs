use super::tree_node::TreeNode;
use htmlentity::entity::{decode, ICodedDataTrait};
use serde::{Deserialize, Serialize};
use xavier::{PError, XmlDeserializable, XmlSerializable};

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Envelope")]
pub struct H40006Object {
    #[xml(tree)]
    pub body: H40006Body,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Body")]
pub struct H40006Body {
    #[xml(tree)]
    pub resp: H40006Resp,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "fund:requestResponse", case = "Camel")]
pub struct H40006Resp {
    #[xml(name = "messageResponseBody")]
    pub orgs: Vec<Org>,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "return")]
pub struct Org {
    #[xml(name = "idt_org__status")]
    pub idt_org_status: i32,
    #[xml(name = "idt_org__id")]
    pub idt_org_id: String,
    #[xml(name = "idt_org__parent_id")]
    pub idt_org_parent_id: String,
    #[xml(name = "idt_org__name")]
    pub idt_org_name: String,
    #[xml(name = "idt_org__sup_org_code")]
    pub idt_org_sup_org_code: String,
    #[xml(name = "idt_org__org_code")]
    pub idt_org_org_code: String,
    //idt_org__iam_oagrgcode
    #[xml(name = "idt_org__iam_oagrgcode")]
    pub idt_org_iam_oagrgcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IOrg {
    pub idt_org_status: i32,
    pub idt_org_id: String,
    pub idt_org_parent_id: String,
    pub idt_org_name: String,
    pub idt_org_real_name: String,
    pub idt_org_sup_org_code: String,
    pub idt_org_org_code: String,
    pub idt_org_iam_oagrgcode: String,
    pub sub_org: Vec<IOrg>,
}

impl TryFrom<Org> for IOrg {
    type Error = anyhow::Error;
    fn try_from(value: Org) -> Result<Self, Self::Error> {
        let nn = decode(value.idt_org_name.as_bytes()).to_string()?;
        Ok(IOrg {
            idt_org_status: value.idt_org_status,
            idt_org_id: value.idt_org_id,
            idt_org_parent_id: value.idt_org_parent_id,
            idt_org_name: value.idt_org_name,
            idt_org_sup_org_code: value.idt_org_sup_org_code,
            idt_org_org_code: value.idt_org_org_code,
            idt_org_real_name: nn,
            idt_org_iam_oagrgcode: value.idt_org_iam_oagrgcode,
            sub_org: Vec::new(),
        })
    }
}

impl TreeNode<String> for IOrg {
    fn get_key(&self) -> String {
        self.idt_org_org_code.clone()
    }
    fn get_parent_key(&self) -> String {
        self.idt_org_sup_org_code.clone()
    }
    fn get_sub_item_mut_ref(&mut self) -> &mut Vec<Self> {
        &mut self.sub_org
    }
    fn take_sub_item(&mut self) -> Vec<Self> {
        std::mem::take(&mut self.sub_org)
    }
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Envelope")]
pub struct H40002Object {
    #[xml(tree)]
    pub body: H40002Body,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Body")]
pub struct H40002Body {
    #[xml(tree)]
    pub resp: H40002Resp,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "fund:requestResponse", case = "Camel")]
pub struct H40002Resp {
    #[xml(tree)]
    pub mb: H40002Mb,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "return")]
pub struct H40002Mb {
    pub access_token: String,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Envelope")]
pub struct H40003Object {
    #[xml(tree)]
    pub body: H40003Body,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Body")]
pub struct H40003Body {
    #[xml(tree)]
    pub resp: H40003Resp,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "fund:requestResponse", case = "Camel")]
pub struct H40003Resp {
    #[xml(tree)]
    pub mb: H40003Mb,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "return")]
pub struct H40003Mb {
    #[xml(name = "responseData")]
    pub user_json: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct H40003User {
    pub id: String,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Envelope")]
pub struct H40005Object {
    #[xml(tree)]
    pub body: H40005Body,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(ns = "soapenv", name = "Body")]
pub struct H40005Body {
    #[xml(tree)]
    pub resp: H40005Resp,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "fund:requestResponse", case = "Camel")]
pub struct H40005Resp {
    #[xml(name = "messageResponseBody")]
    pub users: Vec<User>,
}
/*
<idt_user__mobile>13641948922</idt_user__mobile>
                    <app_account__id/>
                    <idt_user__work_no>014062</idt_user__work_no>
                    <idt_user__job_name>&#31243;&#24207;&#24320;&#21457;&#23703;</idt_user__job_name>
                    <jobs/>
                    <request_log__action_desc>&#31649;&#29702;&#21592;&#25805;&#20316;-&#21024;&#38500;&#24080;&#21495;</request_log__action_desc>
                    <app_account__account_no/>
                    <idt_user__email>sj14062@haitong.com</idt_user__email>
                    <request_log__action_flag>2</request_log__action_flag>
                    <app_account__status/>
                    <useTypes/>
                    <idt_user__job_code>45510</idt_user__job_code>
                    <idt_user__iam_companyid>0000</idt_user__iam_companyid>
                    <orgs>
                        <idt_org__id>1910883477440515052</idt_org__id>
                        <idt_org__name>&#36719;&#20214;&#24320;&#21457;&#22235;&#37096;</idt_org__name>
                        <idt_org__org_code>6327</idt_org__org_code>
                    </orgs>
                    <app_account__account_name/>
                    <idt_user__gender>0</idt_user__gender>
*/
#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "return")]
pub struct User {
    #[xml(name = "idt_user__work_no")]
    pub idt_user_work_no: String,
    #[xml(tree)]
    pub user_org: UserOrg,
}

#[derive(XmlDeserializable, XmlSerializable, Serialize, Deserialize, Debug)]
#[xml(name = "orgs")]
pub struct UserOrg {
    #[xml(name = "idt_org__id")]
    pub idt_org_id: String,
    #[xml(name = "idt_org__name")]
    pub idt_org_name: String,
    #[xml(name = "idt_org__org_code")]
    pub idt_org_org_code: String,
}
