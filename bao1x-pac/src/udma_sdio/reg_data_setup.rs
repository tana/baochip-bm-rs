#[doc = "Register `REG_DATA_SETUP` reader"]
pub type R = crate::R<RegDataSetupSpec>;
#[doc = "Register `REG_DATA_SETUP` writer"]
pub type W = crate::W<RegDataSetupSpec>;
#[doc = "Field `r_data_en` reader - r_data_en"]
pub type RDataEnR = crate::BitReader;
#[doc = "Field `r_data_en` writer - r_data_en"]
pub type RDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_data_rwn` reader - r_data_rwn"]
pub type RDataRwnR = crate::BitReader;
#[doc = "Field `r_data_rwn` writer - r_data_rwn"]
pub type RDataRwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_data_quad` reader - r_data_quad"]
pub type RDataQuadR = crate::BitReader;
#[doc = "Field `r_data_quad` writer - r_data_quad"]
pub type RDataQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_data_block_num` reader - r_data_block_num"]
pub type RDataBlockNumR = crate::FieldReader;
#[doc = "Field `r_data_block_num` writer - r_data_block_num"]
pub type RDataBlockNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_data_block_size` reader - r_data_block_size"]
pub type RDataBlockSizeR = crate::FieldReader<u16>;
#[doc = "Field `r_data_block_size` writer - r_data_block_size"]
pub type RDataBlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - r_data_en"]
    #[inline(always)]
    pub fn r_data_en(&self) -> RDataEnR {
        RDataEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_data_rwn"]
    #[inline(always)]
    pub fn r_data_rwn(&self) -> RDataRwnR {
        RDataRwnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - r_data_quad"]
    #[inline(always)]
    pub fn r_data_quad(&self) -> RDataQuadR {
        RDataQuadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - r_data_block_num"]
    #[inline(always)]
    pub fn r_data_block_num(&self) -> RDataBlockNumR {
        RDataBlockNumR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - r_data_block_size"]
    #[inline(always)]
    pub fn r_data_block_size(&self) -> RDataBlockSizeR {
        RDataBlockSizeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - r_data_en"]
    #[inline(always)]
    pub fn r_data_en(&mut self) -> RDataEnW<'_, RegDataSetupSpec> {
        RDataEnW::new(self, 0)
    }
    #[doc = "Bit 1 - r_data_rwn"]
    #[inline(always)]
    pub fn r_data_rwn(&mut self) -> RDataRwnW<'_, RegDataSetupSpec> {
        RDataRwnW::new(self, 1)
    }
    #[doc = "Bit 2 - r_data_quad"]
    #[inline(always)]
    pub fn r_data_quad(&mut self) -> RDataQuadW<'_, RegDataSetupSpec> {
        RDataQuadW::new(self, 2)
    }
    #[doc = "Bits 8:15 - r_data_block_num"]
    #[inline(always)]
    pub fn r_data_block_num(&mut self) -> RDataBlockNumW<'_, RegDataSetupSpec> {
        RDataBlockNumW::new(self, 8)
    }
    #[doc = "Bits 16:25 - r_data_block_size"]
    #[inline(always)]
    pub fn r_data_block_size(&mut self) -> RDataBlockSizeW<'_, RegDataSetupSpec> {
        RDataBlockSizeW::new(self, 16)
    }
}
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDataSetupSpec;
impl crate::RegisterSpec for RegDataSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_data_setup::R`](R) reader structure"]
impl crate::Readable for RegDataSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_data_setup::W`](W) writer structure"]
impl crate::Writable for RegDataSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DATA_SETUP to value 0"]
impl crate::Resettable for RegDataSetupSpec {}
