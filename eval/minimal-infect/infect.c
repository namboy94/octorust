/*
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octorust.

octorust is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octorust is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octorust.  If not, see <http://www.gnu.org/licenses/>.
*/

#include <octopos.h>
#include <stdio.h>

void signaler(void* sig) {
    simple_signal* s = (simple_signal*)(sig);
    simple_signal_signal_and_exit(s);
}

void ilet_fn(void *signal) {
    printf("Hello World!\n");
    simple_ilet answer;
    simple_ilet_init(&answer, signaler, signal);
    dispatch_claim_send_reply(&answer);
}

void main_ilet(claim_t claim_id) {

    constraints_t constr = agent_constr_create();
    agent_constr_set_quantity(constr, 4, 4, 0);
    agent_constr_set_tile_shareable(constr, 1);
    agentclaim_t claim = agent_claim_invade(0, constr);

    simple_signal sync;
    simple_signal_init(&sync, agent_claim_get_pecount(claim));

    for (int tile=0; tile < get_tile_count(); tile++) {
        int pes=agent_claim_get_pecount_tile_type(claim,  tile, 0);
        if (pes) {
            proxy_claim_t p_claim = 
                agent_claim_get_proxyclaim_tile_type(
                    claim, tile, 0
                );

            simple_ilet ilets[pes];
            for (int i = 0; i < pes; ++i) {
                simple_ilet_init(&ilets[i], ilet_fn, &sync);
            }

            proxy_infect(p_claim, &ilets[0], pes);
        }
    }

    simple_signal_wait(&sync);
    agent_claim_retreat(claim);
    shutdown(0);
}