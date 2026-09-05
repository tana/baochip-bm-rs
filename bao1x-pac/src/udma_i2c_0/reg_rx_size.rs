#[doc = "Register `REG_RX_SIZE` reader"]
pub type R = crate::R<RegRxSizeSpec>;
#[doc = "Register `REG_RX_SIZE` writer"]
pub type W = crate::W<RegRxSizeSpec>;
#[doc = "Field `r_rx_size` reader - r_rx_size"]
pub type RRxSizeR = crate::FieldReader<u16>;
#[doc = "Field `r_rx_size` writer - r_rx_size"]
pub type RRxSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - r_rx_size"]
    #[inline(always)]
    pub fn r_rx_size(&self) -> RRxSizeR {
        RRxSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_rx_size"]
    #[inline(always)]
    pub fn r_rx_size(&mut self) -> RRxSizeW<'_, RegRxSizeSpec> {
        RRxSizeW::new(self, 0)
    }
}
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxSizeSpec;
impl crate::RegisterSpec for RegRxSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_size::R`](R) reader structure"]
impl crate::Readable for RegRxSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_size::W`](W) writer structure"]
impl crate::Writable for RegRxSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_SIZE to value 0"]
impl crate::Resettable for RegRxSizeSpec {}
