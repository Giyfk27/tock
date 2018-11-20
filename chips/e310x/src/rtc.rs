use kernel::common::StaticRef;
use kernel::common::registers::ReadWrite;

pub static mut RTC: Rtc = Rtc::new();

#[repr(C)]
struct RtcRegisters {
	/// RTC Configuration Register
	rtccfg: ReadWrite<u32, rtccfg::Register>,
	_reserved1: [u8; 4],
	/// RTC Counter Low Register
	rtclo: ReadWrite<u32, rtclo::Register>,
	/// RTC Counter High Register
	rtchi: ReadWrite<u32>,
	/// RTC Scaled Counter Register
	rtcs: ReadWrite<u32>,
	_reserved2: [u8; 12],
	/// RTC Compare Register
	rtccmp: ReadWrite<u32, rtccmp::Register>,
}

register_bitfields![u32,
	rtccfg [
	    cmpip OFFSET(28) NUMBITS(1) [],
	    enalways OFFSET(12) NUMBITS(1) [],
	    scale OFFSET(0) NUMBITS(4) []
	],
	rtclo [
		rtclo OFFSET(0) NUMBITS(32) []
	],
	rtchi [
		rtchi OFFSET(0) NUMBITS(16) []
	],
	rtccmp [
		rtccmp OFFSET(0) NUMBITS(32) []
	]
];

const RTC_BASE: StaticRef<RtcRegisters> =
    unsafe { StaticRef::new(0x1000_0040 as *const RtcRegisters) };


pub struct Rtc {
    registers: StaticRef<RtcRegisters>,
}

impl Rtc {
	pub const fn new() -> Rtc {
	    Rtc {
	        registers: RTC_BASE,
	    }
	}

	/// Disable the RTC so it does not generate interrupts.
	pub fn disable(&self) {
		let regs = self.registers;

		// Turn the interrupt compare off so we don't get any RTC interrupts.
		regs.rtccfg.write(rtccfg::enalways::CLEAR);
	}
}
