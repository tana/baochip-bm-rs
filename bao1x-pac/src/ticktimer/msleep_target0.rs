#[doc = "Register `MSLEEP_TARGET0` reader"]
pub type R = crate::R<MsleepTarget0Spec>;
#[doc = "Register `MSLEEP_TARGET0` writer"]
pub type W = crate::W<MsleepTarget0Spec>;
#[doc = "Field `msleep_target` reader - "]
pub type MsleepTargetR = crate::FieldReader<u32>;
#[doc = "Field `msleep_target` writer - "]
pub type MsleepTargetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn msleep_target(&self) -> MsleepTargetR {
        MsleepTargetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn msleep_target(&mut self) -> MsleepTargetW<'_, MsleepTarget0Spec> {
        MsleepTargetW::new(self, 0)
    }
}
#[doc = "Bits 0-31 of `TICKTIMER_MSLEEP_TARGET`.\n\nYou can [`read`](crate::Reg::read) this register and get [`msleep_target0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msleep_target0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsleepTarget0Spec;
impl crate::RegisterSpec for MsleepTarget0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msleep_target0::R`](R) reader structure"]
impl crate::Readable for MsleepTarget0Spec {}
#[doc = "`write(|w| ..)` method takes [`msleep_target0::W`](W) writer structure"]
impl crate::Writable for MsleepTarget0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSLEEP_TARGET0 to value 0"]
impl crate::Resettable for MsleepTarget0Spec {}
