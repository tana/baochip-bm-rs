#[doc = "Register `REG_BINCU_TH` reader"]
pub type R = crate::R<RegBincuThSpec>;
#[doc = "Register `REG_BINCU_TH` writer"]
pub type W = crate::W<RegBincuThSpec>;
#[doc = "Field `r_commit_bincu_threshold` reader - r_commit_bincu_threshold"]
pub type RCommitBincuThresholdR = crate::FieldReader<u32>;
#[doc = "Field `r_commit_bincu_threshold` writer - r_commit_bincu_threshold"]
pub type RCommitBincuThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - r_commit_bincu_threshold"]
    #[inline(always)]
    pub fn r_commit_bincu_threshold(&self) -> RCommitBincuThresholdR {
        RCommitBincuThresholdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - r_commit_bincu_threshold"]
    #[inline(always)]
    pub fn r_commit_bincu_threshold(&mut self) -> RCommitBincuThresholdW<'_, RegBincuThSpec> {
        RCommitBincuThresholdW::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBincuThSpec;
impl crate::RegisterSpec for RegBincuThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bincu_th::R`](R) reader structure"]
impl crate::Readable for RegBincuThSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bincu_th::W`](W) writer structure"]
impl crate::Writable for RegBincuThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BINCU_TH to value 0"]
impl crate::Resettable for RegBincuThSpec {}
