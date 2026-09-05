#[doc = "Register `REG_CLK_DIV` reader"]
pub type R = crate::R<RegClkDivSpec>;
#[doc = "Register `REG_CLK_DIV` writer"]
pub type W = crate::W<RegClkDivSpec>;
#[doc = "Field `r_clk_div_data` reader - r_clk_div_data"]
pub type RClkDivDataR = crate::FieldReader;
#[doc = "Field `r_clk_div_data` writer - r_clk_div_data"]
pub type RClkDivDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_clk_div_valid` reader - r_clk_div_valid"]
pub type RClkDivValidR = crate::BitReader;
#[doc = "Field `r_clk_div_valid` writer - r_clk_div_valid"]
pub type RClkDivValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - r_clk_div_data"]
    #[inline(always)]
    pub fn r_clk_div_data(&self) -> RClkDivDataR {
        RClkDivDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - r_clk_div_valid"]
    #[inline(always)]
    pub fn r_clk_div_valid(&self) -> RClkDivValidR {
        RClkDivValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_clk_div_data"]
    #[inline(always)]
    pub fn r_clk_div_data(&mut self) -> RClkDivDataW<'_, RegClkDivSpec> {
        RClkDivDataW::new(self, 0)
    }
    #[doc = "Bit 8 - r_clk_div_valid"]
    #[inline(always)]
    pub fn r_clk_div_valid(&mut self) -> RClkDivValidW<'_, RegClkDivSpec> {
        RClkDivValidW::new(self, 8)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegClkDivSpec;
impl crate::RegisterSpec for RegClkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clk_div::R`](R) reader structure"]
impl crate::Readable for RegClkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_clk_div::W`](W) writer structure"]
impl crate::Writable for RegClkDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLK_DIV to value 0"]
impl crate::Resettable for RegClkDivSpec {}
