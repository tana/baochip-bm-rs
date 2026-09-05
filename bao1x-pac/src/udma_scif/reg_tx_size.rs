#[doc = "Register `REG_TX_SIZE` reader"]
pub type R = crate::R<RegTxSizeSpec>;
#[doc = "Register `REG_TX_SIZE` writer"]
pub type W = crate::W<RegTxSizeSpec>;
#[doc = "Field `r_tx_size` reader - r_tx_size"]
pub type RTxSizeR = crate::FieldReader<u16>;
#[doc = "Field `r_tx_size` writer - r_tx_size"]
pub type RTxSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - r_tx_size"]
    #[inline(always)]
    pub fn r_tx_size(&self) -> RTxSizeR {
        RTxSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_tx_size"]
    #[inline(always)]
    pub fn r_tx_size(&mut self) -> RTxSizeW<'_, RegTxSizeSpec> {
        RTxSizeW::new(self, 0)
    }
}
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxSizeSpec;
impl crate::RegisterSpec for RegTxSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_size::R`](R) reader structure"]
impl crate::Readable for RegTxSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_size::W`](W) writer structure"]
impl crate::Writable for RegTxSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_SIZE to value 0"]
impl crate::Resettable for RegTxSizeSpec {}
