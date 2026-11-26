#[path = "__basic__/mod.rs"]
mod basic;

pub use basic::card_text::{
    deathtouch, defender, double_strike, enchant, first_strike, flying, haste, hexproof,
    indestructible, infect, lifelink, reach, trample, vigilance,
};
pub use basic::enchant_filter::EnchantFilter;
pub use basic::keyword_id::KeywordId;

#[path = "__traits__/mod.rs"]
pub mod traits;

#[path = "../../__keywords__/mod.rs"]
mod comprehensive_keywords;
pub(crate) use comprehensive_keywords::r702_2c_deathtouch as deathtouch;
pub(crate) use comprehensive_keywords::r702_3a_defender as defender;
pub(crate) use comprehensive_keywords::r702_4b_double_strike as double_strike;
pub(crate) use comprehensive_keywords::r702_5a_enchant as enchant;
pub(crate) use comprehensive_keywords::r702_7b_first_strike as first_strike;
pub(crate) use comprehensive_keywords::r702_9b_flying as flying;
pub(crate) use comprehensive_keywords::r702_10a_haste as haste;
pub(crate) use comprehensive_keywords::r702_11b_hexproof as hexproof;
pub(crate) use comprehensive_keywords::r702_12b_indestructible as indestructible;
pub(crate) use comprehensive_keywords::r702_15b_lifelink as lifelink;
pub(crate) use comprehensive_keywords::r702_17b_reach as reach;
pub(crate) use comprehensive_keywords::r702_19b_r702_19d_trample as trample;
pub(crate) use comprehensive_keywords::r702_20a_vigilance as vigilance;
pub(crate) use comprehensive_keywords::r702_90b_infect as infect;

pub(crate) mod hasher_proxy;
pub(crate) use hasher_proxy::HasherProxy;

mod keyword;
pub use keyword::Keyword;
