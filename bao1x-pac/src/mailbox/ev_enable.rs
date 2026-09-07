#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `available` reader - Write a ``1`` to enable the ``available`` Event"]
pub type AvailableR = crate::BitReader;
#[doc = "Field `available` writer - Write a ``1`` to enable the ``available`` Event"]
pub type AvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_init` reader - Write a ``1`` to enable the ``abort_init`` Event"]
pub type AbortInitR = crate::BitReader;
#[doc = "Field `abort_init` writer - Write a ``1`` to enable the ``abort_init`` Event"]
pub type AbortInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_done` reader - Write a ``1`` to enable the ``abort_done`` Event"]
pub type AbortDoneR = crate::BitReader;
#[doc = "Field `abort_done` writer - Write a ``1`` to enable the ``abort_done`` Event"]
pub type AbortDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error` reader - Write a ``1`` to enable the ``error`` Event"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `error` writer - Write a ``1`` to enable the ``error`` Event"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``available`` Event"]
    #[inline(always)]
    pub fn available(&self) -> AvailableR {
        AvailableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``abort_init`` Event"]
    #[inline(always)]
    pub fn abort_init(&self) -> AbortInitR {
        AbortInitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``abort_done`` Event"]
    #[inline(always)]
    pub fn abort_done(&self) -> AbortDoneR {
        AbortDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``error`` Event"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``available`` Event"]
    #[inline(always)]
    pub fn available(&mut self) -> AvailableW<'_, EvEnableSpec> {
        AvailableW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``abort_init`` Event"]
    #[inline(always)]
    pub fn abort_init(&mut self) -> AbortInitW<'_, EvEnableSpec> {
        AbortInitW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``abort_done`` Event"]
    #[inline(always)]
    pub fn abort_done(&mut self) -> AbortDoneW<'_, EvEnableSpec> {
        AbortDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``error`` Event"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, EvEnableSpec> {
        ErrorW::new(self, 3)
    }
}
#[doc = "Triggers if either `tx_err` or `rx_err` are asserted\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvEnableSpec;
impl crate::RegisterSpec for EvEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EvEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EvEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_ENABLE to value 0"]
impl crate::Resettable for EvEnableSpec {}
