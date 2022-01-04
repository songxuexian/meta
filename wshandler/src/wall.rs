use std::time::Instant;

#[derive(Debug)]
pub(crate) struct ProtectiveWall{
    count: i64,
    last_visit_time: Instant,
}

impl ProtectiveWall {
    fn IsFriendlyVisit(&mut self) -> bool {
        self.count = self.count + 1;
        let cur = Instant::now();
        let dis = cur - self.last_visit_time;
        // 大于等于3秒,重置
        if dis.as_secs() >= 3 {
            self.last_visit_time = cur;
            self.count = 1;
        } else if self.count >= 26 {
            // 如果3秒内访问超过了26次,认为是非法请求
            self.last_visit_time = cur;
            return false;
        }
        true
    }
}
