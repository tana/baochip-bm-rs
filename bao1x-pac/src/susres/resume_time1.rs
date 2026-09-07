#[doc = "Register `RESUME_TIME1` reader"]
pub type R = crate::R<ResumeTime1Spec>;
#[doc = "Register `RESUME_TIME1` writer"]
pub type W = crate::W<ResumeTime1Spec>;
#[doc = "Field `resume_time` reader - "]
pub type ResumeTimeR = crate::FieldReader<u32>;
#[doc = "Field `resume_time` writer - "]
pub type ResumeTimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn resume_time(&self) -> ResumeTimeR {
        ResumeTimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn resume_time(&mut self) -> ResumeTimeW<'_, ResumeTime1Spec> {
        ResumeTimeW::new(self, 0)
    }
}
#[doc = "Bits 32-63 of `SUSRES_RESUME_TIME`. Elapsed time to load. Loaded upon writing `1` to the load bit in the control register. This will immediately affect the msleep extension.\n\nYou can [`read`](crate::Reg::read) this register and get [`resume_time1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resume_time1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResumeTime1Spec;
impl crate::RegisterSpec for ResumeTime1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resume_time1::R`](R) reader structure"]
impl crate::Readable for ResumeTime1Spec {}
#[doc = "`write(|w| ..)` method takes [`resume_time1::W`](W) writer structure"]
impl crate::Writable for ResumeTime1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESUME_TIME1 to value 0"]
impl crate::Resettable for ResumeTime1Spec {}
