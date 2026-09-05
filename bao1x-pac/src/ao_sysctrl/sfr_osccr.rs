#[doc = "Register `SFR_OSCCR` reader"]
pub type R = crate::R<SfrOsccrSpec>;
#[doc = "Register `SFR_OSCCR` writer"]
pub type W = crate::W<SfrOsccrSpec>;
#[doc = "Field `sfrosccr` reader - sfrosccr read/write control register"]
pub type SfrosccrR = crate::BitReader;
#[doc = "Field `sfrosccr` writer - sfrosccr read/write control register"]
pub type SfrosccrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sfrosctrm` reader - sfrosctrm read/write control register"]
pub type SfrosctrmR = crate::BitReader;
#[doc = "Field `sfrosctrm` writer - sfrosctrm read/write control register"]
pub type SfrosctrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sfrosccrlp` reader - sfrosccrlp read/write control register"]
pub type SfrosccrlpR = crate::BitReader;
#[doc = "Field `sfrosccrlp` writer - sfrosccrlp read/write control register"]
pub type SfrosccrlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sfrosctrmlp` reader - sfrosctrmlp read/write control register"]
pub type SfrosctrmlpR = crate::BitReader;
#[doc = "Field `sfrosctrmlp` writer - sfrosctrmlp read/write control register"]
pub type SfrosctrmlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sfrosccrpd` reader - sfrosccrpd read/write control register"]
pub type SfrosccrpdR = crate::BitReader;
#[doc = "Field `sfrosccrpd` writer - sfrosccrpd read/write control register"]
pub type SfrosccrpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sfrosccr read/write control register"]
    #[inline(always)]
    pub fn sfrosccr(&self) -> SfrosccrR {
        SfrosccrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sfrosctrm read/write control register"]
    #[inline(always)]
    pub fn sfrosctrm(&self) -> SfrosctrmR {
        SfrosctrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sfrosccrlp read/write control register"]
    #[inline(always)]
    pub fn sfrosccrlp(&self) -> SfrosccrlpR {
        SfrosccrlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sfrosctrmlp read/write control register"]
    #[inline(always)]
    pub fn sfrosctrmlp(&self) -> SfrosctrmlpR {
        SfrosctrmlpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sfrosccrpd read/write control register"]
    #[inline(always)]
    pub fn sfrosccrpd(&self) -> SfrosccrpdR {
        SfrosccrpdR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sfrosccr read/write control register"]
    #[inline(always)]
    pub fn sfrosccr(&mut self) -> SfrosccrW<'_, SfrOsccrSpec> {
        SfrosccrW::new(self, 0)
    }
    #[doc = "Bit 1 - sfrosctrm read/write control register"]
    #[inline(always)]
    pub fn sfrosctrm(&mut self) -> SfrosctrmW<'_, SfrOsccrSpec> {
        SfrosctrmW::new(self, 1)
    }
    #[doc = "Bit 2 - sfrosccrlp read/write control register"]
    #[inline(always)]
    pub fn sfrosccrlp(&mut self) -> SfrosccrlpW<'_, SfrOsccrSpec> {
        SfrosccrlpW::new(self, 2)
    }
    #[doc = "Bit 3 - sfrosctrmlp read/write control register"]
    #[inline(always)]
    pub fn sfrosctrmlp(&mut self) -> SfrosctrmlpW<'_, SfrOsccrSpec> {
        SfrosctrmlpW::new(self, 3)
    }
    #[doc = "Bit 4 - sfrosccrpd read/write control register"]
    #[inline(always)]
    pub fn sfrosccrpd(&mut self) -> SfrosccrpdW<'_, SfrOsccrSpec> {
        SfrosccrpdW::new(self, 4)
    }
}
#[doc = "See `ao_sysctrl.sv#L386 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L386>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_osccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_osccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOsccrSpec;
impl crate::RegisterSpec for SfrOsccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_osccr::R`](R) reader structure"]
impl crate::Readable for SfrOsccrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_osccr::W`](W) writer structure"]
impl crate::Writable for SfrOsccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OSCCR to value 0"]
impl crate::Resettable for SfrOsccrSpec {}
