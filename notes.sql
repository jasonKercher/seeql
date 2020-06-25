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
     t1.seq t1_joinkey, t2.seq t2_joinkey -- only need 1
    ,package t1_value, package + 'x' t2_value
    ,identity(int, 1, 1) xxxxxx_ID
into #TEST
from table1 t1
left join table2 t2 on t1.seq = t2.seq
where package = ''

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


