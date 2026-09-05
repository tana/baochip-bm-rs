#[doc = "Register `REG_AU_REG0` reader"]
pub type R = crate::R<RegAuReg0Spec>;
#[doc = "Register `REG_AU_REG0` writer"]
pub type W = crate::W<RegAuReg0Spec>;
#[doc = "Field `r_commit_au_reg0` reader - r_commit_au_reg0"]
pub type RCommitAuReg0R = crate::FieldReader<u32>;
#[doc = "Field `r_commit_au_reg0` writer - r_commit_au_reg0"]
pub type RCommitAuReg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - r_commit_au_reg0"]
    #[inline(always)]
    pub fn r_commit_au_reg0(&self) -> RCommitAuReg0R {
        RCommitAuReg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - r_commit_au_reg0"]
    #[inline(always)]
    pub fn r_commit_au_reg0(&mut self) -> RCommitAuReg0W<'_, RegAuReg0Spec> {
        RCommitAuReg0W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_au_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_au_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAuReg0Spec;
impl crate::RegisterSpec for RegAuReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_au_reg0::R`](R) reader structure"]
impl crate::Readable for RegAuReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_au_reg0::W`](W) writer structure"]
impl crate::Writable for RegAuReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AU_REG0 to value 0"]
impl crate::Resettable for RegAuReg0Spec {}
