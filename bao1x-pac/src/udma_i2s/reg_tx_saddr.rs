#[doc = "Register `REG_TX_SADDR` reader"]
pub type R = crate::R<RegTxSaddrSpec>;
#[doc = "Register `REG_TX_SADDR` writer"]
pub type W = crate::W<RegTxSaddrSpec>;
#[doc = "Field `r_tx_startaddr` reader - r_tx_startaddr"]
pub type RTxStartaddrR = crate::FieldReader<u16>;
#[doc = "Field `r_tx_startaddr` writer - r_tx_startaddr"]
pub type RTxStartaddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - r_tx_startaddr"]
    #[inline(always)]
    pub fn r_tx_startaddr(&self) -> RTxStartaddrR {
        RTxStartaddrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - r_tx_startaddr"]
    #[inline(always)]
    pub fn r_tx_startaddr(&mut self) -> RTxStartaddrW<'_, RegTxSaddrSpec> {
        RTxStartaddrW::new(self, 0)
    }
}
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_saddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_saddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxSaddrSpec;
impl crate::RegisterSpec for RegTxSaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_saddr::R`](R) reader structure"]
impl crate::Readable for RegTxSaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_saddr::W`](W) writer structure"]
impl crate::Writable for RegTxSaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_SADDR to value 0"]
impl crate::Resettable for RegTxSaddrSpec {}
