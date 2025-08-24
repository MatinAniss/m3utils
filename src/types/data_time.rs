#[derive(Debug)]
pub struct DateTime {
    pub date: DateTimeDate,
    pub time: DateTimeTime,
    pub time_zone: Option<DateTimeTimezone>,
}

#[derive(Debug)]
pub struct DateTimeDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug)]
pub struct DateTimeTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
}

#[derive(Debug)]
pub struct DateTimeTimezone {
    pub hour_offset: DateTimeTimezoneHourOffset,
    pub minute_offset: u8,
}

#[derive(Debug)]
pub enum DateTimeTimezoneHourOffset {
    East(u8),
    West(u8),
}
