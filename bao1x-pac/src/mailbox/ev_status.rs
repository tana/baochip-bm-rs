#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `available` reader - Level of the ``available`` event"]
pub type AvailableR = crate::BitReader;
#[doc = "Field `available` writer - Level of the ``available`` event"]
pub type AvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_init` reader - Level of the ``abort_init`` event"]
pub type AbortInitR = crate::BitReader;
#[doc = "Field `abort_init` writer - Level of the ``abort_init`` event"]
pub type AbortInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_done` reader - Level of the ``abort_done`` event"]
pub type AbortDoneR = crate::BitReader;
#[doc = "Field `abort_done` writer - Level of the ``abort_done`` event"]
pub type AbortDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error` reader - Level of the ``error`` event"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `error` writer - Level of the ``error`` event"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``available`` event"]
    #[inline(always)]
    pub fn available(&self) -> AvailableR {
        AvailableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``abort_init`` event"]
    #[inline(always)]
    pub fn abort_init(&self) -> AbortInitR {
        AbortInitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``abort_done`` event"]
    #[inline(always)]
    pub fn abort_done(&self) -> AbortDoneR {
        AbortDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``error`` event"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``available`` event"]
    #[inline(always)]
    pub fn available(&mut self) -> AvailableW<'_, EvStatusSpec> {
        AvailableW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``abort_init`` event"]
    #[inline(always)]
    pub fn abort_init(&mut self) -> AbortInitW<'_, EvStatusSpec> {
        AbortInitW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``abort_done`` event"]
    #[inline(always)]
    pub fn abort_done(&mut self) -> AbortDoneW<'_, EvStatusSpec> {
        AbortDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``error`` event"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, EvStatusSpec> {
        ErrorW::new(self, 3)
    }
}
#[doc = "Triggers if either `tx_err` or `rx_err` are asserted\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
