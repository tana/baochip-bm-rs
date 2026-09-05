#[doc = "Register `REG_AU_REG1` reader"]
pub type R = crate::R<RegAuReg1Spec>;
#[doc = "Register `REG_AU_REG1` writer"]
pub type W = crate::W<RegAuReg1Spec>;
#[doc = "Field `r_commit_au_reg1` reader - r_commit_au_reg1"]
pub type RCommitAuReg1R = crate::FieldReader<u32>;
#[doc = "Field `r_commit_au_reg1` writer - r_commit_au_reg1"]
pub type RCommitAuReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - r_commit_au_reg1"]
    #[inline(always)]
    pub fn r_commit_au_reg1(&self) -> RCommitAuReg1R {
        RCommitAuReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - r_commit_au_reg1"]
    #[inline(always)]
    pub fn r_commit_au_reg1(&mut self) -> RCommitAuReg1W<'_, RegAuReg1Spec> {
        RCommitAuReg1W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_au_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_au_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAuReg1Spec;
impl crate::RegisterSpec for RegAuReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_au_reg1::R`](R) reader structure"]
impl crate::Readable for RegAuReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_au_reg1::W`](W) writer structure"]
impl crate::Writable for RegAuReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AU_REG1 to value 0"]
impl crate::Resettable for RegAuReg1Spec {}
