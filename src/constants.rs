use once_cell::sync::Lazy;
use serenity::all::ChannelId;

pub static DELETE_THRESHOLD: u16 = 850;
pub static WARN_THRESHOLD: u16 = 1950;

pub static MOD_LOG_CHANNEL: Lazy<ChannelId> = Lazy::new(|| {
    ChannelId::new(1113711421839130664)
});

pub static DEBUG_LOG_CHANNEL: Lazy<ChannelId> = Lazy::new(|| {
    ChannelId::new(1170136488038637599)
});