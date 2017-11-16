/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octolib.

octolib is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octolib is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octolib.  If not, see <http://www.gnu.org/licenses/>.
*/

/*
/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_event.h file
*/

/* TODO
extern void octo_event_enable();
extern void octo_event_disable();

///
///\brief schedule the specified event on an available unit
///
///if there are no suitable units available on the hardware, this function
///may return a nullptr.
///
extern struct octo_event *octo_event_schedule(const struct octo_event_attr *attr);

///
///\brief start the previously scheduled event
///
extern bool octo_event_start(struct octo_event *);

///
///\brief stop the previously started event
///
extern bool octo_event_stop(struct octo_event *);

///
///\brief suspend a started event
///
extern bool octo_event_suspend(struct octo_event *);

///
///\brief resume a started event
///
extern bool octo_event_resume (struct octo_event *);

///
///\brief release the hardware counters of this handle
///
///the handle will remain valid until octo_event_destroy is called. Also
///a possibly allocated buffer (for sample records) is valid and not released
///until a call to octo_event_destroy. This method only allows the hardware to
///be reused by other events///
extern bool octo_event_release(struct octo_event *);

///
///\brief send the samples over the select transports
///
extern bool octo_event_send(const struct octo_event *);

///
///\brief get the record buffer
///
///\note the buffer will be only valid as long as the event had not been destroyed!
///
extern bool octo_event_records(const struct octo_event *, const struct octo_event_record **buffer, uint64_t *count);

///
///\brief get the current value
///
extern uint64_t octo_event_read(const struct octo_event *);

///
///\brief destroy the event and release all resources
///
///the buffer returned by octo_event_records is invalidated!///
extern void octo_event_destroy(struct octo_event *);
*/