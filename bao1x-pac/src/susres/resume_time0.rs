#[doc = "Register `RESUME_TIME0` reader"]
pub type R = crate::R<ResumeTime0Spec>;
#[doc = "Register `RESUME_TIME0` writer"]
pub type W = crate::W<ResumeTime0Spec>;
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
    pub fn resume_time(&mut self) -> ResumeTimeW<'_, ResumeTime0Spec> {
        ResumeTimeW::new(self, 0)
    }
}
#[doc = "Bits 0-31 of `SUSRES_RESUME_TIME`.\n\nYou can [`read`](crate::Reg::read) this register and get [`resume_time0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resume_time0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResumeTime0Spec;
impl crate::RegisterSpec for ResumeTime0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resume_time0::R`](R) reader structure"]
impl crate::Readable for ResumeTime0Spec {}
#[doc = "`write(|w| ..)` method takes [`resume_time0::W`](W) writer structure"]
impl crate::Writable for ResumeTime0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESUME_TIME0 to value 0"]
impl crate::Resettable for ResumeTime0Spec {}
