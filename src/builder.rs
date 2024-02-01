use std::io::Write;

pub struct IronLoggerBuilder<W:Write>
{
    styles:Option<LoggerStyles>,        //定义的等级
    file:Option<W>,                     //文件写入泛型
    capacity:Option<usize>,             //一级缓存的最大大小
    time_setting:Option<TimeSetting>    //时间设置，如区域等
}

impl<W:Write> IronLoggerBuilder<W>
{
    pub fn new() -> Self
    {
        Self
        {
            styles:None,
            file:None,
            capacity:None,
            time_setting:None
        }
    }
}