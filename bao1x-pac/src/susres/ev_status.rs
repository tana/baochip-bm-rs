#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `soft_int` reader - Level of the ``soft_int`` event"]
pub type SoftIntR = crate::BitReader;
#[doc = "Field `soft_int` writer - Level of the ``soft_int`` event"]
pub type SoftIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``soft_int`` event"]
    #[inline(always)]
    pub fn soft_int(&self) -> SoftIntR {
        SoftIntR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``soft_int`` event"]
    #[inline(always)]
    pub fn soft_int(&mut self) -> SoftIntW<'_, EvStatusSpec> {
        SoftIntW::new(self, 0)
    }
}
#[doc = "This register contains the current raw level of the soft_int event trigger. Writes to this register have no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvStatusSpec;
impl crate::RegisterSpec for EvStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_status::R`](R) reader structure"]
impl crate::Readable for EvStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_status::W`](W) writer structure"]
impl crate::Writable for EvStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_STATUS to value 0"]
impl crate::Resettable for EvStatusSpec {}
