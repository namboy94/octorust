#ifndef _OCTO_EVENT_H_
#define _OCTO_EVENT_H_
#include "octo_event_types.h"
#if defined(__cplusplus) || defined(c_plusplus)
extern "C" {
#endif

extern void octo_event_enable ();
extern void octo_event_disable ();

/**
 * @brief schedule the specified event on an available unit
 *
 * if there are no suitable units available on the hardware, this function
 * may return a nullptr. */
extern struct octo_event *octo_event_schedule (const struct octo_event_attr *attr);

/**
 * @brief start the previously scheduled event
 */
extern bool octo_event_start (struct octo_event *);

/**
 * @brief stop the previously started event
 */
extern bool octo_event_stop (struct octo_event *);

/**
 * @brief suspend a started event
 */
extern bool octo_event_suspend (struct octo_event *);

/**
 * @brief resume a started event
 */
extern bool octo_event_resume (struct octo_event *);

/**
 * @brief release the hardware counters of this handle
 *
 * the handle will remain valid until octo_event_destroy is called. Also
 * a possibly allocated buffer (for sample records) is valid and not released
 * until a call to octo_event_destroy. This method only allows the hardware to
 * be reused by other events */
extern bool octo_event_release (struct octo_event *);

/**
 * @brief send the samples over the select transports
 */
extern bool octo_event_send (const struct octo_event *);

/**
 * @brief get the record buffer
 *
 * NOTE: the buffer will be only valid as long as the event had not been destroyed!
 */
extern bool octo_event_records (const struct octo_event *, const struct octo_event_record **buffer, uint64_t *count);

/**
 * @brief get the current value
 */
extern uint64_t octo_event_read (const struct octo_event *);

/**
 * @brief destroy the event and release all resources
 *
 * the buffer returned by octo_event_records is invalidated! */
extern void octo_event_destroy (struct octo_event *);

#if defined(__cplusplus) || defined(c_plusplus)
}
#endif
#endif /* !_OCTO_EVENT_H_ */
