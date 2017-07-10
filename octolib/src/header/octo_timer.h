#ifndef _OCTO_TIMER_H_
#define _OCTO_TIMER_H_

#include <inttypes.h>

#ifdef __cplusplus
extern "C" {
#endif

uint64_t timer_start(void);

uint64_t timer_stop(void);

uint64_t ticks_to_nanoseconds(uint64_t ticks);

#ifdef __cplusplus
}
#endif

#endif /* _OCTO_TIMER_H_ */
