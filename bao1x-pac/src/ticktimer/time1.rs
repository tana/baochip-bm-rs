#[doc = "Register `TIME1` reader"]
pub type R = crate::R<Time1Spec>;
#[doc = "Register `TIME1` writer"]
pub type W = crate::W<Time1Spec>;
#[doc = "Field `time` reader - "]
pub type TimeR = crate::FieldReader<u32>;
#[doc = "Field `time` writer - "]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn time(&mut self) -> TimeW<'_, Time1Spec> {
        TimeW::new(self, 0)
    }
}
#[doc = "Bits 32-63 of `TICKTIMER_TIME`. Elapsed time in systicks\n\nYou can [`read`](crate::Reg::read) this register and get [`time1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Time1Spec;
impl crate::RegisterSpec for Time1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time1::R`](R) reader structure"]
impl crate::Readable for Time1Spec {}
#[doc = "`write(|w| ..)` method takes [`time1::W`](W) writer structure"]
impl crate::Writable for Time1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIME1 to value 0"]
impl crate::Resettable for Time1Spec {}
