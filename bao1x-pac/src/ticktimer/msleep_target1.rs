#[doc = "Register `MSLEEP_TARGET1` reader"]
pub type R = crate::R<MsleepTarget1Spec>;
#[doc = "Register `MSLEEP_TARGET1` writer"]
pub type W = crate::W<MsleepTarget1Spec>;
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
    pub fn msleep_target(&mut self) -> MsleepTargetW<'_, MsleepTarget1Spec> {
        MsleepTargetW::new(self, 0)
    }
}
#[doc = "Bits 32-63 of `TICKTIMER_MSLEEP_TARGET`. Target time in 1.0ms ticks\n\nYou can [`read`](crate::Reg::read) this register and get [`msleep_target1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msleep_target1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsleepTarget1Spec;
impl crate::RegisterSpec for MsleepTarget1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msleep_target1::R`](R) reader structure"]
impl crate::Readable for MsleepTarget1Spec {}
#[doc = "`write(|w| ..)` method takes [`msleep_target1::W`](W) writer structure"]
impl crate::Writable for MsleepTarget1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSLEEP_TARGET1 to value 0"]
impl crate::Resettable for MsleepTarget1Spec {}
