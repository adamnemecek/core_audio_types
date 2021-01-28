// /*!
// 	@file		AudioSessionTypes.h
// 	@framework	CoreAudioTypes.framework
// 	@copyright	(c) 2009-2019 by Apple, Inc., all rights reserved.
//     @abstract   Definitions for types used by AVAudioSession.
// */
// #ifndef CoreAudioTypes_AudioSessionTypes_h
// #define CoreAudioTypes_AudioSessionTypes_h

// #include <CoreFoundation/CFAvailability.h>
// #include <stdint.h>

// // Types which are binary-compatible with NS(U)Integer but without pulling in objc headers.
// #if __LP64__ || 0 || NS_BUILD_32_LIKE_64
// typedef long AVAudioInteger;
// typedef unsigned long AVAudioUInteger;
// #else
// typedef int AVAudioInteger;
// typedef unsigned int AVAudioUInteger;
// #endif

// #ifdef __OBJC__
//  // Needed for apinotes.
// @class AVAudioSession;
// #endif

// /*!
// 	@typedef	AudioSessionID
// 	@brief		Defines a unique identifier for an audio session.
// */
// typedef uint32_t AudioSessionID;
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AudioSessionID(u32);

// /*!
//     @enum		AVAudioSessionErrorCode
//     @brief		Error codes returned from the AVAudioSession API.
//     @var        AVAudioSessionErrorCodeNone
//         Operation succeeded.
//     @var        AVAudioSessionErrorCodeMediaServicesFailed
//         The app attempted to use the audio session during or after a Media Services failure.  App
//  		should wait for a AVAudioSessionMediaServicesWereResetNotification and then rebuild all
//  		its state.
//     @var        AVAudioSessionErrorCodeIsBusy
//         The app attempted to set its audio session inactive or change its AVAudioSessionIOType,
//  		but it is still actively playing and/or recording.
//     @var        AVAudioSessionErrorCodeIncompatibleCategory
//         The app tried to perform an operation on a session but its category does not support it.
//  		For instance, if the app calls setPreferredInputNumberOfChannels: while in a playback-only
//  		category.
//     @var        AVAudioSessionErrorCodeCannotInterruptOthers
//         The app's audio session is non-mixable and trying to go active while in the background.
//  		This is allowed only when the app is the NowPlaying app.
//     @var        AVAudioSessionErrorCodeMissingEntitlement
//         The app does not have the required entitlements to perform an operation.
//     @var        AVAudioSessionErrorCodeSiriIsRecording
//         The app tried to do something with the audio session that is not allowed while Siri is
//  		recording.
//     @var        AVAudioSessionErrorCodeCannotStartPlaying
//         The app is not allowed to start recording and/or playing, usually because of a lack of audio
//  		key in its Info.plist.  This could also happen if the app has this key but uses a category
//  		that can't record and/or play in the background (AVAudioSessionCategoryAmbient,
//  		AVAudioSessionCategorySoloAmbient, etc.).
//     @var        AVAudioSessionErrorCodeCannotStartRecording
//         The app is not allowed to start recording, usually because it is starting a mixable
//  		recording from the background and is not an Inter-App Audio app.
//     @var        AVAudioSessionErrorCodeBadParam
//         An illegal value was used for a property.
//     @var        AVAudioSessionErrorInsufficientPriority
//         The app was not allowed to set the audio category because another app (Phone, etc.) is
//  		controlling it.
//     @var        AVAudioSessionErrorCodeResourceNotAvailable
//         The operation failed because the device does not have sufficient hardware resources to
//         complete the action. For example, the operation requires audio input hardware, but the
//         device has no audio input available.
//     @var         AVAudioSessionErrorCodeExpiredSession
//         The operation failed because the associated session has been destroyed.
//     @var        AVAudioSessionErrorCodeUnspecified
//         An unspecified error has occurred.
// 	@var		AVAudioSessionErrorCodeSessionNotActive
// 		The operation failed because the session is not active.
//  */
// typedef CF_ENUM(AVAudioInteger, AVAudioSessionErrorCode) {
//     AVAudioSessionErrorCodeNone = 0,
//     AVAudioSessionErrorCodeMediaServicesFailed = 'msrv',   // 0x6D737276, 1836282486
//     AVAudioSessionErrorCodeIsBusy = '!act',                // 0x21616374, 560030580
//     AVAudioSessionErrorCodeIncompatibleCategory = '!cat',  // 0x21636174, 560161140
//     AVAudioSessionErrorCodeCannotInterruptOthers = '!int', // 0x21696E74, 560557684
//     AVAudioSessionErrorCodeMissingEntitlement = 'ent?',    // 0x656E743F, 1701737535
//     AVAudioSessionErrorCodeSiriIsRecording = 'siri',       // 0x73697269, 1936290409
//     AVAudioSessionErrorCodeCannotStartPlaying = '!pla',    // 0x21706C61, 561015905
//     AVAudioSessionErrorCodeCannotStartRecording = '!rec',  // 0x21726563, 561145187
//     AVAudioSessionErrorCodeBadParam = -50,
//     AVAudioSessionErrorCodeInsufficientPriority = '!pri',  // 0x21707269, 561017449
//     AVAudioSessionErrorCodeResourceNotAvailable = '!res',  // 0x21726573, 561145203
//     AVAudioSessionErrorCodeUnspecified = 'what',           // 0x77686174, 2003329396
//     AVAudioSessionErrorCodeExpiredSession = '!ses',        // 0x21736573, 561210739
//     AVAudioSessionErrorCodeSessionNotActive = 'inac',      // 0x696e6163, 1768841571
// };

const fn four_cc(cc: &[u8; 4]) -> u64 {
    cc4::four_cc(cc) as u64
}

const fn cast(a: i32) -> u64 {
    let a: u32 = unsafe { std::mem::transmute(a) };
    a as u64
}
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, Hash,)]
pub enum AVAudioSessionErrorCode {
    None = 0,
    MediaServicesFailed = four_cc(b"msrv"),   // 0x6D737276, 1836282486
    IsBusy = four_cc(b"!act"),                // 0x21616374, 560030580
    IncompatibleCategory = four_cc(b"!cat"),  // 0x21636174, 560161140
    CannotInterruptOthers = four_cc(b"!int"), // 0x21696E74, 560557684
    MissingEntitlement = four_cc(b"ent?"),    // 0x656E743F, 1701737535
    SiriIsRecording = four_cc(b"siri"),       // 0x73697269, 1936290409
    CannotStartPlaying = four_cc(b"!pla"),    // 0x21706C61, 561015905
    CannotStartRecording = four_cc(b"!rec"),  // 0x21726563, 561145187
    BadParam = cast(-50),
    InsufficientPriority = four_cc(b"!pri"),  // 0x21707269, 561017449
    ResourceNotAvailable = four_cc(b"!res"),  // 0x21726573, 561145203
    Unspecified = four_cc(b"what"),           // 0x77686174, 2003329396
    ExpiredSession = four_cc(b"!ses"),        // 0x21736573, 561210739
    SessionNotActive = four_cc(b"inac"),      // 0x696e6163, 1768841571
}

// enum {
//     AVAudioSessionErrorInsufficientPriority API_DEPRECATED_WITH_REPLACEMENT("AVAudioSessionErrorCodeInsufficientPriority", ios(7.0, 12.0)) API_UNAVAILABLE(macos) = AVAudioSessionErrorCodeInsufficientPriority
// };

// #endif // CoreAudioTypes_AudioSessionTypes_h
