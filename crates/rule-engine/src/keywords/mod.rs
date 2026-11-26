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
pub use comprehensive_keywords::deathtouch::Deathtouch;
pub use comprehensive_keywords::defender::Defender;
pub use comprehensive_keywords::double_strike::DoubleStrike;
pub use comprehensive_keywords::enchant::Enchant;
pub use comprehensive_keywords::first_strike::FirstStrike;
pub use comprehensive_keywords::flying::Flying;
pub use comprehensive_keywords::haste::Haste;
pub use comprehensive_keywords::hexproof::Hexproof;
pub use comprehensive_keywords::indestructible::Indestructible;
pub use comprehensive_keywords::infect::Infect;
pub use comprehensive_keywords::lifelink::Lifelink;
pub use comprehensive_keywords::reach::Reach;
pub use comprehensive_keywords::trample::Trample;
pub use comprehensive_keywords::vigilance::Vigilance;

pub(crate) mod hasher_proxy;
pub(crate) use hasher_proxy::HasherProxy;

mod keyword;
pub use keyword::Keyword;
