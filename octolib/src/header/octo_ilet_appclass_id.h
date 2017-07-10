#ifndef _OCTO_ILET_APPCLASS_ID_H_
#define _OCTO_ILET_APPCLASS_ID_H_

#include "octo_ilet.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 *  \brief Initialise simple ilet(one parameter form) including id and applclass
 *  \param ilet Pointer to ilet instance
 *  \param code Pointer to function
 *  \param param parameter for function
 *  \param id id to be written into ilet
 *  \param appl_class application clas to be written into ilet
 */
void simple_ilet_init_appclass(simple_ilet* ilet, ilet_func code, void* param, uint32_t id, uint32_t appl_class);

/**
 *  \brief Initialise dual ilet(two parameter form) including id and applclass
 *  \param ilet Pointer to ilet instance
 *  \param code Pointer to function
 *  \param param1 first parameter for function
 *  \param param2 second parameter for function
 *  \param id id to be written into ilet
 *  \param appl_class application clas to be written into ilet
 */
void dual_ilet_init_appclass(simple_ilet* ilet, dual_ilet_func code, void* param1, void* param2, uint32_t id, uint32_t appl_class);

/**
 *  \brief Program monitor weights of an application class
 *  \param class_index Index of application class
 *  \param class_weights Every nibble represents a weight for the corresponding
 *  monitor value
 *  \return 0 on success, -1 on failure
 */
int program_application_class(uint32_t class_index, uint32_t class_weights);

#ifdef __cplusplus
}
#endif

#endif
