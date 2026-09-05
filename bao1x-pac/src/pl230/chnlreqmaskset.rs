#[doc = "Register `CHNLREQMASKSET` reader"]
pub type R = crate::R<ChnlreqmasksetSpec>;
#[doc = "Register `CHNLREQMASKSET` writer"]
pub type W = crate::W<ChnlreqmasksetSpec>;
#[doc = "Field `CHNL_REQ_MASK_SET` reader - CHNL_REQ_MASK_SET"]
pub type ChnlReqMaskSetR = crate::FieldReader;
#[doc = "Field `CHNL_REQ_MASK_SET` writer - CHNL_REQ_MASK_SET"]
pub type ChnlReqMaskSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CHNL_REQ_MASK_SET"]
    #[inline(always)]
    pub fn chnl_req_mask_set(&self) -> ChnlReqMaskSetR {
        ChnlReqMaskSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CHNL_REQ_MASK_SET"]
    #[inline(always)]
    pub fn chnl_req_mask_set(&mut self) -> ChnlReqMaskSetW<'_, ChnlreqmasksetSpec> {
        ChnlReqMaskSetW::new(self, 0)
    }
}
#[doc = "DMA Channel Request Mask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlreqmaskset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlreqmaskset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlreqmasksetSpec;
impl crate::RegisterSpec for ChnlreqmasksetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnlreqmaskset::R`](R) reader structure"]
impl crate::Readable for ChnlreqmasksetSpec {}
#[doc = "`write(|w| ..)` method takes [`chnlreqmaskset::W`](W) writer structure"]
impl crate::Writable for ChnlreqmasksetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLREQMASKSET to value 0"]
impl crate::Resettable for ChnlreqmasksetSpec {}
