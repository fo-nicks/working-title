# Operational Design

Something that I hadn't completely thought through is how things should 
work from an operational perspective.  At the highest level, I want things to 
'just work' - you set things up to be backed up and you don't have to think 
about it until either (a) something is preventing the backup from succeeding 
(leaving you exposed) or (b) you need to restore data from the backup.
I think as an operator, there are three categories of activities - configuring 
the backup, viewing or being notified of the status of the backup, and restoring
from the backup.

# Configuration

I have a lot of uncertainty here.  The thing I think I know - you will need 
to decide how much space you want to share and that dictates how much you can 
backup (or you can go the other way, the amount you want to backup dictates how 
much you must share).  The storage allocation should happen as early as possible 
to guarantee that it doesn't get used by other data. The big uncertainty - how do 
you configure/discover your peers?  I think the fundamental question is whether 
the relationship between peers is transitive - if A is a peer of B and B is a peer
of C, is A a peer of C?  If the answer is yes, I think this implies that the peer
group is a relatively small, new peers are added to the group by invitation, and 
all peers know about all others.  It would be possible for there to be sort of 
collective group agreement.  If the answer is no, everyone has their own set of 
peers, and no one knows who anyone else's peers are.  A collective agreement wouldn't 
be possible here, as there is no group.  I'm leaning towards having peer groups 
(i.e. answer is yes), at least initially, as I think it makes things operationally simpler.
With that leaning, operationally, I think this means that a peer just starts its 
synchronization process and allows other potential peers to connect.  The first 
connection between potential peers is in the form of a request to join the group.  
(The assumption is that the connection is only possible with some sort of 
invitation - e.g. I tell you how to connect to me.)  The request (by the operator) is reviewed and either accepted or rejected.  If accepted, the potential peer becomes an actual peer, and every member of the group is updated.  Note, in this model, anyone is able to add new peers. which I think is fine initially.
If peers can be added, they can also be removed.  This is a potential land mine, 
but I think initially, anyone can remove a peer, including the peer itself.  The 
assumption here is that no one is going to be malicious.  The removal gets distributed to all of the members of the group.
The collective agreement is a big part of this - every member of the group shares 
in the same agreement.  Initially, I think all this agreement dictates is that you must contribute n * x storage if you want to back up a max of x.  (n is the fault tolerance).
Finally, the files to be backed up need to be configured.  Initially, I think this 
is as simple as a list of directories.

In summary, the proposed up-front configuration items:
- Connection information for the background process.
- List of directories to backup.
- Collective agreement (i.e. fault tolerance)
- Where to allocated storage from

On-going configuration items:
- Members of the peer group
- Status

I think that the various pieces of status are tracked by the sync process.  Things that  might be interesting:
Are all of the files backed up? (If not, when?)
- How much can I back up with the agreed upon tolerance with the current peers?  (The problem here is that there may be asymmetry between the amount of storage contributed by various peers - e.g. if a peer A contributes 10 TB with three other peers contributing 10 GB, the big peer will be a bit limited.)
- How much space is my current backup occupying?
- How much of my contributed space is currently used?  (Maybe I reduce my contribution if not all of it is used.)
- Who are all of the peers?

# Recovery

I think the initial goal to deal with catastrophe - we don't look to restore individual 
files.  (Or more specifically, if you accidentally deleted a file, you wouldn't look to 
the backup.)  If you suffer a drive failure, the backed up contents can be recovered, 
and those contents will be as close as possible to the state of those files as possible 
as the time of failure.

If a catastrophic failure occurred, what does the operator need to have and what do 
they need to do?  Assuming everything is encrypted, the public and private keys will 
be necessary.  And since every peer knows about all others, they need one peer to 
connect to, and then everything is known.  It'd be ideal if that's all they need to have.
What do they need to do?  Another bit of uncertainty, but it'd be nice to say that 
all they have to do is start the background process, and it would know to start 
recovering, but I'm not sure how possible this is.  The alternative is to start it in a 
recovery mode, but that seems problematic also - what if it doesn't get started in 
recovery mode when it should have?  Will that have the effect of clearing the backup?
It'd be nice to rely on something like inotify - if we can assume that we see all changes 
to files, then I think we can know if we should be recovering v. syncing.  That is, if 
the backup contains a bunch of files that aren't in the current directories, we can assume 
that the user didn't remove them and they should be recovered.  (Consequently, if we could 
work this way, we'd have essentially made a distributed backup and sync service, which 
multiple computers could be kept in sync via the backup machinery.)  But there are lots 
of dragons here... I think the initial go at this should assume they will start in a 
recovery mode, with a check for an empty directory and non-empty backup.

# Strategy

There does need to be a strategy, but the less agreement required between the peers 
the simpler things will be.  I'm thinking it'd be ideal to have the peers operate 
as autonomously as possible - the magic will be in designing an algorithm such that 
peers can, as long as they know how who all of the peers are and how much storage is 
available on each, will do a reasonable thing (I don't think maximally best is a 
requirement).  I don't know if this will be possible - I don't have such an algorithm 
at the ready - but it feels like it should be possible.  Again, this will have to 
assume that there isn't malicious intent by any of the peers, which I think is also 
safe to assume at this point.