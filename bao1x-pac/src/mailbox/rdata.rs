#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RdataSpec>;
#[doc = "Register `RDATA` writer"]
pub type W = crate::W<RdataSpec>;
#[doc = "Field `rdata` reader - "]
pub type RdataR = crate::FieldReader<u32>;
#[doc = "Field `rdata` writer - "]
pub type RdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdata(&mut self) -> RdataW<'_, RdataSpec> {
        RdataW::new(self, 0)
    }
}
#[doc = "Read data from incoming FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`rdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdataSpec;
impl crate::RegisterSpec for RdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata::R`](R) reader structure"]
impl crate::Readable for RdataSpec {}
#[doc = "`write(|w| ..)` method takes [`rdata::W`](W) writer structure"]
impl crate::Writable for RdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RdataSpec {}
