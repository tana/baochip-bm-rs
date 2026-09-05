#[doc = "Register `REG_RX_SADDR` reader"]
pub type R = crate::R<RegRxSaddrSpec>;
#[doc = "Register `REG_RX_SADDR` writer"]
pub type W = crate::W<RegRxSaddrSpec>;
#[doc = "Field `r_rx_startaddr` reader - r_rx_startaddr"]
pub type RRxStartaddrR = crate::FieldReader<u16>;
#[doc = "Field `r_rx_startaddr` writer - r_rx_startaddr"]
pub type RRxStartaddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - r_rx_startaddr"]
    #[inline(always)]
    pub fn r_rx_startaddr(&self) -> RRxStartaddrR {
        RRxStartaddrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - r_rx_startaddr"]
    #[inline(always)]
    pub fn r_rx_startaddr(&mut self) -> RRxStartaddrW<'_, RegRxSaddrSpec> {
        RRxStartaddrW::new(self, 0)
    }
}
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_saddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_saddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxSaddrSpec;
impl crate::RegisterSpec for RegRxSaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_saddr::R`](R) reader structure"]
impl crate::Readable for RegRxSaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_saddr::W`](W) writer structure"]
impl crate::Writable for RegRxSaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_SADDR to value 0"]
impl crate::Resettable for RegRxSaddrSpec {}
