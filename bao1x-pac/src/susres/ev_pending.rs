#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `soft_int` reader - `1` if a `soft_int` event occurred. This Event is triggered on a **falling** edge."]
pub type SoftIntR = crate::BitReader;
#[doc = "Field `soft_int` writer - `1` if a `soft_int` event occurred. This Event is triggered on a **falling** edge."]
pub type SoftIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` if a `soft_int` event occurred. This Event is triggered on a **falling** edge."]
    #[inline(always)]
    pub fn soft_int(&self) -> SoftIntR {
        SoftIntR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` if a `soft_int` event occurred. This Event is triggered on a **falling** edge."]
    #[inline(always)]
    pub fn soft_int(&mut self) -> SoftIntW<'_, EvPendingSpec> {
        SoftIntW::new(self, 0)
    }
}
#[doc = "When a soft_int event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvPendingSpec;
impl crate::RegisterSpec for EvPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EvPendingSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EvPendingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_PENDING to value 0"]
impl crate::Resettable for EvPendingSpec {}
