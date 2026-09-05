#[doc = "Register `REG_START` reader"]
pub type R = crate::R<RegStartSpec>;
#[doc = "Register `REG_START` writer"]
pub type W = crate::W<RegStartSpec>;
#[doc = "Field `r_sdio_start` reader - r_sdio_start"]
pub type RSdioStartR = crate::BitReader;
#[doc = "Field `r_sdio_start` writer - r_sdio_start"]
pub type RSdioStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_sdio_start"]
    #[inline(always)]
    pub fn r_sdio_start(&self) -> RSdioStartR {
        RSdioStartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_sdio_start"]
    #[inline(always)]
    pub fn r_sdio_start(&mut self) -> RSdioStartW<'_, RegStartSpec> {
        RSdioStartW::new(self, 0)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegStartSpec;
impl crate::RegisterSpec for RegStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_start::R`](R) reader structure"]
impl crate::Readable for RegStartSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_start::W`](W) writer structure"]
impl crate::Writable for RegStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_START to value 0"]
impl crate::Resettable for RegStartSpec {}
