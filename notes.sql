/**
 * Update
 */

/** Input **/
update t1
set package = package + 'x'
from table1 t1
join table2 t2 on t1.seq = t2.seq
where package = ''

/** Output **/
select
    package org_value,
    package + 'x' new_value
into #TEST
from table1 t1
join table2 t2 on t1.seq = t2.seq
where package = ''

declare @${hash}_unjoined bigint = (
    select count(*)
    from table1 t1
    left join table2 t2 on t1.seq = t2.seq
    where package = ''
        and t2.seq is null
)

/** Tracking table **/
insert into tracking
select query, hash, field, changed, affected, unaffected, unjoined

/**
 * Output
 *
 * Test
 * Qty changed
 * Qty affected
 * Qty unjoined (join only)
 *
 *
 * Warnings
 * no where clause ~ okay if t2 is smaller (< 10%)
 *
 */
