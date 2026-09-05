#[doc = "Register `REG_VALID` reader"]
pub type R = crate::R<RegValidSpec>;
#[doc = "Register `REG_VALID` writer"]
pub type W = crate::W<RegValidSpec>;
#[doc = "Field `r_scif_rx_data_valid` reader - r_scif_rx_data_valid"]
pub type RScifRxDataValidR = crate::BitReader;
#[doc = "Field `r_scif_rx_data_valid` writer - r_scif_rx_data_valid"]
pub type RScifRxDataValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_scif_rx_data_valid"]
    #[inline(always)]
    pub fn r_scif_rx_data_valid(&self) -> RScifRxDataValidR {
        RScifRxDataValidR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_scif_rx_data_valid"]
    #[inline(always)]
    pub fn r_scif_rx_data_valid(&mut self) -> RScifRxDataValidW<'_, RegValidSpec> {
        RScifRxDataValidW::new(self, 0)
    }
}
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_valid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_valid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegValidSpec;
impl crate::RegisterSpec for RegValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_valid::R`](R) reader structure"]
impl crate::Readable for RegValidSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_valid::W`](W) writer structure"]
impl crate::Writable for RegValidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VALID to value 0"]
impl crate::Resettable for RegValidSpec {}
