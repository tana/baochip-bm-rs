#[doc = "Register `SFR_OPT` reader"]
pub type R = crate::R<SfrOptSpec>;
#[doc = "Register `SFR_OPT` writer"]
pub type W = crate::W<SfrOptSpec>;
#[doc = "Field `opt_klen0` reader - opt_klen0 read/write control register"]
pub type OptKlen0R = crate::FieldReader;
#[doc = "Field `opt_klen0` writer - opt_klen0 read/write control register"]
pub type OptKlen0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `opt_mode0` reader - opt_mode0 read/write control register"]
pub type OptMode0R = crate::FieldReader;
#[doc = "Field `opt_mode0` writer - opt_mode0 read/write control register"]
pub type OptMode0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `opt_ifstart0` reader - opt_ifstart0 read/write control register"]
pub type OptIfstart0R = crate::BitReader;
#[doc = "Field `opt_ifstart0` writer - opt_ifstart0 read/write control register"]
pub type OptIfstart0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - opt_klen0 read/write control register"]
    #[inline(always)]
    pub fn opt_klen0(&self) -> OptKlen0R {
        OptKlen0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - opt_mode0 read/write control register"]
    #[inline(always)]
    pub fn opt_mode0(&self) -> OptMode0R {
        OptMode0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - opt_ifstart0 read/write control register"]
    #[inline(always)]
    pub fn opt_ifstart0(&self) -> OptIfstart0R {
        OptIfstart0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - opt_klen0 read/write control register"]
    #[inline(always)]
    pub fn opt_klen0(&mut self) -> OptKlen0W<'_, SfrOptSpec> {
        OptKlen0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - opt_mode0 read/write control register"]
    #[inline(always)]
    pub fn opt_mode0(&mut self) -> OptMode0W<'_, SfrOptSpec> {
        OptMode0W::new(self, 4)
    }
    #[doc = "Bit 8 - opt_ifstart0 read/write control register"]
    #[inline(always)]
    pub fn opt_ifstart0(&mut self) -> OptIfstart0W<'_, SfrOptSpec> {
        OptIfstart0W::new(self, 8)
    }
}
#[doc = "See `aes.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L145>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOptSpec;
impl crate::RegisterSpec for SfrOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_opt::R`](R) reader structure"]
impl crate::Readable for SfrOptSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_opt::W`](W) writer structure"]
impl crate::Writable for SfrOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPT to value 0"]
impl crate::Resettable for SfrOptSpec {}
