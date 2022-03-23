use std::fmt::Display;

use rust_decimal::prelude::*;
//use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_utc_time_ms::{de_string_to_utc_time_ms, se_time_ms_to_utc_string};
use time_ms_conversions::time_ms_to_utc_string;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, PartialOrd, Ord)]
#[serde(rename_all = "PascalCase")]
pub enum TokenTaxRecType {
    Income,
    Deposit,
    Mining,
    Gift,
    Trade,
    Withdrawal,
    Spend,
    Lost,
    Stolen,
    Unknown,
}

impl Display for TokenTaxRecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub enum GroupType {
    #[serde(rename = "margin")]
    Margin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TokenTaxRec {
    #[serde(rename = "Type")]
    pub type_txs: TokenTaxRecType,

    pub buy_amount: Option<Decimal>,
    pub buy_currency: String,
    pub sell_amount: Option<Decimal>,
    pub sell_currency: String,
    pub fee_amount: Option<Decimal>,
    pub fee_currency: String,
    pub exchange: String,
    pub group: Option<GroupType>,
    pub comment: String,

    #[serde(rename = "Date")]
    #[serde(deserialize_with = "de_string_to_utc_time_ms")]
    #[serde(serialize_with = "se_time_ms_to_utc_string")]
    pub time: i64,
}

impl TokenTaxRec {
    pub fn new() -> TokenTaxRec {
        TokenTaxRec {
            type_txs: TokenTaxRecType::Unknown,
            buy_amount: None,
            buy_currency: "".to_string(),
            sell_amount: None,
            sell_currency: "".to_string(),
            fee_amount: None,
            fee_currency: "".to_string(),
            exchange: "".to_string(),
            group: None,
            comment: "".to_string(),
            time: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from(
        type_txs: TokenTaxRecType,
        buy_amount: Option<Decimal>,
        buy_currency: String,
        sell_amount: Option<Decimal>,
        sell_currency: String,
        fee_amount: Option<Decimal>,
        fee_currency: String,
        exchange: String,
        group: Option<GroupType>,
        comment: String,
        time: i64,
    ) -> TokenTaxRec {
        TokenTaxRec {
            type_txs,
            buy_amount,
            buy_currency,
            sell_amount,
            sell_currency,
            fee_amount,
            fee_currency,
            exchange,
            group,
            comment,
            time,
        }
    }

    pub fn get_asset(&self) -> &str {
        match self.type_txs {
            TokenTaxRecType::Unknown => panic!("SNH"),
            TokenTaxRecType::Trade => &self.buy_currency,
            TokenTaxRecType::Deposit => &self.buy_currency,
            TokenTaxRecType::Withdrawal => &self.sell_currency,
            TokenTaxRecType::Income => &self.buy_currency,
            TokenTaxRecType::Spend => &self.sell_currency,
            TokenTaxRecType::Lost => &self.sell_currency,
            TokenTaxRecType::Stolen => &self.sell_currency,
            TokenTaxRecType::Mining => &self.buy_currency,
            TokenTaxRecType::Gift => &self.sell_currency,
        }
    }

    pub fn get_quantity(&self) -> Decimal {
        match self.type_txs {
            TokenTaxRecType::Unknown => panic!("SNH"),
            TokenTaxRecType::Trade => self.buy_amount.expect("SNH"),
            TokenTaxRecType::Deposit => self.buy_amount.expect("SNH"),
            TokenTaxRecType::Withdrawal => self.sell_amount.expect("SNH"),
            TokenTaxRecType::Income => self.buy_amount.expect("SNH"),
            TokenTaxRecType::Spend => self.sell_amount.expect("SNH"),
            TokenTaxRecType::Lost => self.sell_amount.expect("SNH"),
            TokenTaxRecType::Stolen => self.sell_amount.expect("SNH"),
            TokenTaxRecType::Mining => self.buy_amount.expect("SNH"),
            TokenTaxRecType::Gift => self.sell_amount.expect("SNH"),
        }
    }

    pub fn get_other_asset(&self) -> &str {
        match self.type_txs {
            TokenTaxRecType::Unknown => panic!("SNH"),
            TokenTaxRecType::Trade => &self.sell_currency,
            TokenTaxRecType::Deposit => &self.sell_currency,
            TokenTaxRecType::Withdrawal => &self.buy_currency,
            TokenTaxRecType::Income => &self.sell_currency,
            TokenTaxRecType::Spend => &self.buy_currency,
            TokenTaxRecType::Lost => &self.buy_currency,
            TokenTaxRecType::Stolen => &self.buy_currency,
            TokenTaxRecType::Mining => &self.sell_currency,
            TokenTaxRecType::Gift => &self.buy_currency,
        }
    }
}

impl Default for TokenTaxRec {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for TokenTaxRec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "time: {} type_txs: {} buy_amount: {:?} buy_currency {} sell_amount: {:?} sell_currency: {} fee_amount: {:?} fee_currency: {} exchange: {} group: {:?} comment: {}",
            time_ms_to_utc_string(self.time),
            self.type_txs,
            self.buy_amount,
            self.buy_currency,
            self.sell_amount,
            self.sell_currency,
            self.fee_amount,
            self.fee_currency,
            self.exchange,
            self.group,
            self.comment,
        )
    }
}

// TODO: Add tests for Eq Ord PartialEq PartialOrd
impl Eq for TokenTaxRec {}

// Manually imiplement PartialEq so time is sorted first
impl PartialEq for TokenTaxRec {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
            && self.type_txs == other.type_txs
            && self.buy_currency == other.buy_currency
            && self.sell_currency == other.sell_currency
            && self.fee_currency == other.fee_currency
            && self.buy_amount == other.buy_amount
            && self.sell_amount == other.sell_amount
            && self.fee_amount == other.fee_amount
            && self.exchange == other.exchange
            && self.group == other.group
            && self.comment == other.comment
    }
}

impl PartialOrd for TokenTaxRec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.time.partial_cmp(&other.time) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.type_txs.partial_cmp(&other.type_txs) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.buy_currency.partial_cmp(&other.buy_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sell_currency.partial_cmp(&other.sell_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.fee_currency.partial_cmp(&other.fee_currency) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.buy_amount.partial_cmp(&other.buy_amount) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sell_amount.partial_cmp(&other.sell_amount) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.fee_amount.partial_cmp(&other.fee_amount) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.exchange.partial_cmp(&other.exchange) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.group.partial_cmp(&other.group) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.comment.partial_cmp(&other.comment)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rust_decimal_macros::dec;
    #[test]
    fn test_new() {
        let ttr = TokenTaxRec::new();
        assert_eq!(ttr.type_txs, TokenTaxRecType::Unknown);
        assert_eq!(ttr.buy_amount, None);
        assert_eq!(ttr.buy_currency, "".to_owned());
        assert_eq!(ttr.sell_amount, None);
        assert_eq!(ttr.sell_currency, "".to_owned());
        assert_eq!(ttr.fee_amount, None);
        assert_eq!(ttr.fee_currency, "".to_owned());
        assert_eq!(ttr.exchange, "".to_owned());
        assert_eq!(ttr.group, None);
        assert_eq!(ttr.comment, "".to_owned());
    }

    #[test]
    fn test_default() {
        let ttr_default = TokenTaxRec::default();
        let ttr_new = TokenTaxRec::new();
        assert_eq!(ttr_default, ttr_new);
    }

    #[test]
    fn test_eqne() {
        let mut ttr = TokenTaxRec::default();
        let mut ttr_other = TokenTaxRec::default();
        assert!(ttr == ttr_other);

        // The order is important so we go though all the paths,
        // so we modifiy the last test first        ttr.comment = "a".to_owned();
        ttr_other.comment = "b".to_owned();
        assert!(ttr != ttr_other);

        ttr.exchange = "a".to_owned();
        ttr_other.exchange = "b".to_owned();
        assert!(ttr != ttr_other);

        ttr.fee_amount = Some(dec!(0));
        ttr_other.fee_amount = Some(dec!(1));
        assert!(ttr != ttr_other);

        ttr.sell_amount = Some(dec!(0));
        ttr_other.sell_amount = Some(dec!(1));
        assert!(ttr != ttr_other);

        ttr.buy_amount = Some(dec!(0));
        ttr_other.buy_amount = Some(dec!(1));
        assert!(ttr != ttr_other);

        ttr.fee_currency = "a".to_owned();
        ttr_other.fee_currency = "b".to_owned();
        assert!(ttr != ttr_other);

        ttr.sell_currency = "a".to_owned();
        ttr_other.sell_currency = "b".to_owned();
        assert!(ttr != ttr_other);

        ttr.buy_currency = "a".to_owned();
        ttr_other.buy_currency = "b".to_owned();
        assert!(ttr != ttr_other);

        ttr.type_txs = TokenTaxRecType::Income;
        ttr_other.type_txs = TokenTaxRecType::Trade;
        assert!(ttr != ttr_other);

        ttr.time = 0;
        ttr_other.time = 1;
        assert!(ttr != ttr_other);
    }

    #[test]
    fn test_partial_ord() {
        let mut ttr = TokenTaxRec::default();
        let mut ttr_other = TokenTaxRec::default();

        assert!(ttr <= ttr_other);

        // The order is important so we go though all the paths,
        // so we modifiy the last test first        ttr.comment = "a".to_owned();
        ttr_other.comment = "b".to_owned();
        assert!(ttr < ttr_other);

        ttr.exchange = "a".to_owned();
        ttr_other.exchange = "b".to_owned();
        assert!(ttr < ttr_other);

        ttr.fee_amount = Some(dec!(0));
        ttr_other.fee_amount = Some(dec!(1));
        assert!(ttr < ttr_other);

        ttr.sell_amount = Some(dec!(0));
        ttr_other.sell_amount = Some(dec!(1));
        assert!(ttr < ttr_other);

        ttr.buy_amount = Some(dec!(0));
        ttr_other.buy_amount = Some(dec!(1));
        assert!(ttr < ttr_other);

        ttr.fee_currency = "a".to_owned();
        ttr_other.fee_currency = "b".to_owned();
        assert!(ttr < ttr_other);

        ttr.sell_currency = "a".to_owned();
        ttr_other.sell_currency = "b".to_owned();
        assert!(ttr < ttr_other);

        ttr.buy_currency = "a".to_owned();
        ttr_other.buy_currency = "b".to_owned();
        assert!(ttr < ttr_other);

        ttr.type_txs = TokenTaxRecType::Income;
        ttr_other.type_txs = TokenTaxRecType::Trade;
        assert!(ttr < ttr_other);

        ttr.time = 0;
        ttr_other.time = 1;
        assert!(ttr < ttr_other);
    }

    #[test]
    #[should_panic]
    fn test_get_asset_panic() {
        let tbr = TokenTaxRec::new();

        assert_eq!(tbr.type_txs, TokenTaxRecType::Unknown);
        tbr.get_asset();
    }

    #[test]
    fn test_get_asset() {
        let mut tbr = TokenTaxRec::new();

        tbr.type_txs = TokenTaxRecType::Withdrawal;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Spend;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Stolen;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Gift;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Lost;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Trade;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Deposit;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Income;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Mining;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_asset(), "ABC");
    }

    #[test]
    #[should_panic]
    fn test_get_other_asset_panic() {
        let tbr = TokenTaxRec::new();

        assert_eq!(tbr.type_txs, TokenTaxRecType::Unknown);
        tbr.get_other_asset();
    }

    #[test]
    fn test_get_other_asset() {
        let mut tbr = TokenTaxRec::new();

        tbr.type_txs = TokenTaxRecType::Withdrawal;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Spend;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Stolen;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Gift;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Lost;
        tbr.buy_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Trade;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Deposit;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Income;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");

        tbr.type_txs = TokenTaxRecType::Mining;
        tbr.sell_currency = "ABC".to_owned();
        assert_eq!(tbr.get_other_asset(), "ABC");
    }

    #[test]
    #[should_panic]
    fn test_get_quantity_panic() {
        let tbr = TokenTaxRec::new();

        assert_eq!(tbr.type_txs, TokenTaxRecType::Unknown);
        tbr.get_quantity();
    }

    #[test]
    fn test_get_quantity() {
        let mut tbr = TokenTaxRec::new();

        tbr.type_txs = TokenTaxRecType::Withdrawal;
        tbr.sell_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Spend;
        tbr.sell_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Stolen;
        tbr.sell_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Gift;
        tbr.sell_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Lost;
        tbr.sell_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Trade;
        tbr.buy_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Deposit;
        tbr.buy_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Income;
        tbr.buy_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));

        tbr.type_txs = TokenTaxRecType::Mining;
        tbr.buy_amount = Some(dec!(1));
        assert_eq!(tbr.get_quantity(), dec!(1));
    }

    #[test]
    fn test_deserialize_from_csv() {
        let csv = "

Type,BuyAmount,BuyCurrency,SellAmount,SellCurrency,FeeAmount,FeeCurrency,Exchange,Group,Comment,Date
Deposit,5125,USD,,,,,binance.us,,,1970-01-01 00:00:00 
Trade,1,ETH,3123.00,USD,0.00124,BNB,binance.us,,,1970-01-01 00:00:00 
Trade,1,ETH,312.00,USD,0.00124,BNB,binance.us,margin,,1970-01-01 00:00:00 
Income,0.001,BNB,,,,,binance.us,,\"Referral Commission\",1970-01-01 00:00:00 
Withdrawal,,,100,USD,,,some bank,,\"AccountId: 123456\",1970-01-01 00:00:00 
Spend,,,100,USD,0.01,USD,,,\"Gift for wife\",1970-01-01 00:00:00 
Lost,,,1,ETH,,,,,\"Wallet lost\",1970-01-01 00:00:00 
Stolen,,,1,USD,,,,,\"Wallet hacked\",1970-01-01 00:00:00 
Mining,0.000002,ETH,,,,,binance.us,,\"ETH2 validator reward\",1970-01-01 00:00:00 
Gift,,,100,USD,,,,,\"Gift to friend\",1970-01-01 00:00:00 
";

        let rdr = csv.as_bytes();
        let mut reader = csv::Reader::from_reader(rdr);
        for (idx, entry) in reader.deserialize().enumerate() {
            println!("{idx}: entry: {:?}", entry);
            match entry {
                Ok(rec) => {
                    let ttcr: TokenTaxRec = rec;
                    println!("tr: {:?}", ttcr);
                    match idx {
                        0 => {
                            // Deposit,5125,USD,,,,,binance.us,,,1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Deposit);
                            assert_eq!(ttcr.buy_amount, Some(dec!(5125)));
                            assert_eq!(ttcr.buy_currency, "USD");
                            assert_eq!(ttcr.sell_amount, None);
                            assert_eq!(ttcr.sell_currency, "");
                            assert_eq!(ttcr.fee_amount, None);
                            assert_eq!(ttcr.fee_currency, "");
                            assert_eq!(ttcr.exchange, "binance.us");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "");
                            assert_eq!(ttcr.time, 0);
                        }
                        1 => {
                            // Trade,1,ETH,3123.00,USD,0.00124,BNB,binance.us,,,1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Trade);
                            assert_eq!(ttcr.buy_amount, Some(dec!(1)));
                            assert_eq!(ttcr.buy_currency, "ETH");
                            assert_eq!(ttcr.sell_amount, Some(dec!(3123)));
                            assert_eq!(ttcr.sell_currency, "USD");
                            assert_eq!(ttcr.fee_amount, Some(dec!(0.00124)));
                            assert_eq!(ttcr.fee_currency, "BNB");
                            assert_eq!(ttcr.exchange, "binance.us");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "");
                            assert_eq!(ttcr.time, 0);
                        }
                        2 => {
                            // Trade,1,ETH,312.00,USD,0.00124,BNB,binance.us,margin,,1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Trade);
                            assert_eq!(ttcr.buy_amount, Some(dec!(1)));
                            assert_eq!(ttcr.buy_currency, "ETH");
                            assert_eq!(ttcr.sell_amount, Some(dec!(312)));
                            assert_eq!(ttcr.sell_currency, "USD");
                            assert_eq!(ttcr.fee_amount, Some(dec!(0.00124)));
                            assert_eq!(ttcr.fee_currency, "BNB");
                            assert_eq!(ttcr.exchange, "binance.us");
                            assert_eq!(ttcr.group, Some(GroupType::Margin));
                            assert_eq!(ttcr.comment, "");
                            assert_eq!(ttcr.time, 0);
                        }
                        3 => {
                            // Income,0.001,BNB,,,,,binance.us,,\"Referral Commission\",1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Income);
                            assert_eq!(ttcr.buy_amount, Some(dec!(0.001)));
                            assert_eq!(ttcr.buy_currency, "BNB");
                            assert_eq!(ttcr.sell_amount, None);
                            assert_eq!(ttcr.sell_currency, "");
                            assert_eq!(ttcr.fee_amount, None);
                            assert_eq!(ttcr.fee_currency, "");
                            assert_eq!(ttcr.exchange, "binance.us");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "Referral Commission");
                            assert_eq!(ttcr.time, 0);
                        }
                        4 => {
                            // Withdrawal,,,100,USD,,,some bank,,\"AccountId: 123456\",1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Withdrawal);
                            assert_eq!(ttcr.buy_amount, None);
                            assert_eq!(ttcr.buy_currency, "");
                            assert_eq!(ttcr.sell_amount, Some(dec!(100)));
                            assert_eq!(ttcr.sell_currency, "USD");
                            assert_eq!(ttcr.fee_amount, None);
                            assert_eq!(ttcr.fee_currency, "");
                            assert_eq!(ttcr.exchange, "some bank");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "AccountId: 123456");
                            assert_eq!(ttcr.time, 0);
                        }
                        5 => {
                            // Spend,,,100,USD,0.01,USD,,,\"Gift for wife\",1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Spend);
                            assert_eq!(ttcr.buy_amount, None);
                            assert_eq!(ttcr.buy_currency, "");
                            assert_eq!(ttcr.sell_amount, Some(dec!(100)));
                            assert_eq!(ttcr.sell_currency, "USD");
                            assert_eq!(ttcr.fee_amount, Some(dec!(0.01)));
                            assert_eq!(ttcr.fee_currency, "USD");
                            assert_eq!(ttcr.exchange, "");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "Gift for wife");
                            assert_eq!(ttcr.time, 0);
                        }
                        6 => {
                            // Lost,,,1,ETH,,,,,\"Wallet lost\",1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Lost);
                            assert_eq!(ttcr.buy_amount, None);
                            assert_eq!(ttcr.buy_currency, "");
                            assert_eq!(ttcr.sell_amount, Some(dec!(1)));
                            assert_eq!(ttcr.sell_currency, "ETH");
                            assert_eq!(ttcr.fee_amount, None);
                            assert_eq!(ttcr.fee_currency, "");
                            assert_eq!(ttcr.exchange, "");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "Wallet lost");
                            assert_eq!(ttcr.time, 0);
                        }
                        7 => {
                            // Stolen,,,1,USD,,,,,\"Wallet hacked\",1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Stolen);
                            assert_eq!(ttcr.buy_amount, None);
                            assert_eq!(ttcr.buy_currency, "");
                            assert_eq!(ttcr.sell_amount, Some(dec!(1)));
                            assert_eq!(ttcr.sell_currency, "USD");
                            assert_eq!(ttcr.fee_amount, None);
                            assert_eq!(ttcr.fee_currency, "");
                            assert_eq!(ttcr.exchange, "");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "Wallet hacked");
                            assert_eq!(ttcr.time, 0);
                        }
                        8 => {
                            // Mining,0.000002,ETH,,,,,binance.us,,\"ETH2 validator reward\",1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Mining);
                            assert_eq!(ttcr.buy_amount, Some(dec!(0.000002)));
                            assert_eq!(ttcr.buy_currency, "ETH");
                            assert_eq!(ttcr.sell_amount, None);
                            assert_eq!(ttcr.sell_currency, "");
                            assert_eq!(ttcr.fee_amount, None);
                            assert_eq!(ttcr.fee_currency, "");
                            assert_eq!(ttcr.exchange, "binance.us");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "ETH2 validator reward");
                            assert_eq!(ttcr.time, 0);
                        }
                        9 => {
                            // Gift,,,100,USD,,,,,\"Gift to friend\",1970-01-01 00:00:00
                            assert_eq!(ttcr.type_txs, TokenTaxRecType::Gift);
                            assert_eq!(ttcr.buy_amount, None);
                            assert_eq!(ttcr.buy_currency, "");
                            assert_eq!(ttcr.sell_amount, Some(dec!(100)));
                            assert_eq!(ttcr.sell_currency, "USD");
                            assert_eq!(ttcr.fee_amount, None);
                            assert_eq!(ttcr.fee_currency, "");
                            assert_eq!(ttcr.exchange, "");
                            assert_eq!(ttcr.group, None);
                            assert_eq!(ttcr.comment, "Gift to friend");
                            assert_eq!(ttcr.time, 0);
                        }
                        _ => panic!("Unexpected idx"),
                    }
                }
                Err(e) => panic!("Error: {e}"),
            }
        }
    }
}
