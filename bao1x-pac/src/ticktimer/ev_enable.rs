#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `alarm` reader - Write a ``1`` to enable the ``alarm`` Event"]
pub type AlarmR = crate::BitReader;
#[doc = "Field `alarm` writer - Write a ``1`` to enable the ``alarm`` Event"]
pub type AlarmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``alarm`` Event"]
    #[inline(always)]
    pub fn alarm(&self) -> AlarmR {
        AlarmR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``alarm`` Event"]
    #[inline(always)]
    pub fn alarm(&mut self) -> AlarmW<'_, EvEnableSpec> {
        AlarmW::new(self, 0)
    }
}
#[doc = "This register enables the corresponding alarm events. Write a ``0`` to this register to disable individual events.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
