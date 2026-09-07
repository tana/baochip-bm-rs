#[doc = "Register `HEARTBEAT` reader"]
pub type R = crate::R<HeartbeatSpec>;
#[doc = "Register `HEARTBEAT` writer"]
pub type W = crate::W<HeartbeatSpec>;
#[doc = "Field `beat` reader - Set to `1` at the next `count` interval rollover since `clear` was set."]
pub type BeatR = crate::BitReader;
#[doc = "Field `beat` writer - Set to `1` at the next `count` interval rollover since `clear` was set."]
pub type BeatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to `1` at the next `count` interval rollover since `clear` was set."]
    #[inline(always)]
    pub fn beat(&self) -> BeatR {
        BeatR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to `1` at the next `count` interval rollover since `clear` was set."]
    #[inline(always)]
    pub fn beat(&mut self) -> BeatW<'_, HeartbeatSpec> {
        BeatW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`heartbeat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`heartbeat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeartbeatSpec;
impl crate::RegisterSpec for HeartbeatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`heartbeat::R`](R) reader structure"]
impl crate::Readable for HeartbeatSpec {}
#[doc = "`write(|w| ..)` method takes [`heartbeat::W`](W) writer structure"]
impl crate::Writable for HeartbeatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HEARTBEAT to value 0"]
impl crate::Resettable for HeartbeatSpec {}
