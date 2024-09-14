# XHSTT Genetic Algorithm

## XHSTT Problem Instances
Timetabling problems in the XHSTT XML format can be downloaded
[here](https://www.utwente.nl/en/eemcs/dmmp/hstt/archives/).

Files are available as collections (containing multiple problem instances)
and as single-instance files. Currently, single-instance files are preferred,
as they also come with the known solutions.

To get started with implementing a scheduler, the artificial problem instances
have been chosen, so it's not needed to implement a whole lot of constraints.

# XHSTT
## Datastructure
```sh
HighSchoolTimetableArchive [+Id]
    + MetaData
        Name
        Contributor
        Date
        Description
        + Remarks

    + Instances
        * Instance [Id]
            MetaData
                Name
                Contributor
                Date
                Country
                Description
                + Remarks

            Times
                + TimeGroups
                    * Week [Id]
                        Name
                        # + TimeGroup ???

                    * Day [Id]
                        Name
                        # + TimeGroup ???

                    * TimeGroup [Id]
                        Name

                * Time [Id]
                    Name
                    + Week [Reference]
                    + Day [Reference]
                    + TimeGroups
                        * TimeGroup [Reference]

            Resources
                + ResourceTypes
                    * ResourceType [Id]
                        Name

                + ResourceGroups
                    * ResourceGroup [Id]
                        Name
                        ResourceType [Reference]

                * Resource [Id]
                    Name
                    ResourceType [Reference]
                    + ResourceGroups
                        * ResourceGroup [Reference]

            Events
                + EventGroups
                    * Course [Id]
                        Name
                        # + EventGroup ??? (similar to week, day and timegroup)

                    * EventGroup [Id]
                        Name

                * Event [Id] [+Color]
                    Name
                    Duration (int >= 1)
                    + Workload
                    + Course [Reference]
                    + Time [Reference]
                    + Resources
                        * Resource [+Reference]
                            + Role (body = value)
                            + ResourceType [Reference] (not documented well)
                            + Workload

                    + ResourceGroups
                        * ResourceGroup [Reference]

                    + EventGroups
                        * EventGroup [Reference]

            Constraints
                Name
                Required
                ...

                AppliesTo (events; both)
                    + EventGroups
                        * EventGroup [Reference]

                    + Events
                        * Event [Reference]

                AppliesTo (events; groups only)
                    EventGroups
                        * EventGroup [Reference]


                AppliesTo (events; pairs)
                    EventPairs
                        * EventPair
                            FirstEvent [Reference]
                            SecondEvent [Reference]
                            + MinSeparation
                            + MaxSeparation

                AppliesTo (resources; both)
                    + ResourceGroups
                        * ResourceGroup [Reference]

                    + Resources
                        * Resource [Reference]

    + SolutionGroups
        * SolutionGroup [Id]
            MetaData
                Contributor
                Date
                Description
                + Publication
                + Remarks

            * Solution [Reference]
                + Description
                + RunningTime
                + Events
                    * Event [Reference]
                        + Duration
                        + Time [Reference]
                        + Ressources
                            * Resource [Reference]
                                Role
                + Report
                    InfeasibilityValue
                    ObjectiveValue
                    + Resources
                        * Resource [Reference]
                            * Constraint [Reference]
                                Cost
                                + Description
                    + Events
                        * Event [Reference]
                            * Constraint [Reference]
                                Cost
                                + Description

                    + EventGroups
                        * EventGroup [Reference]
                            * Constraint [Reference]
                                Cost
                                + Description

```

## Instances
|               | Times | Needs Resource assignment | Needs Class assignment    |
|---------------|-------|---------------------------|---------------------------|
| **XHSTT2014** |       |                           |                           |
| AU-BG-98      |    40 | yes                       | no                        |
| AU-SA-96      |    60 | yes                       | no                        |
| AU-TE-99      |    30 | yes                       | no                        |
| BR-SA-00      |    25 | no                        | no                        |
| BR-SM-00      |    25 | no                        | no                        |
| BR-SN-00      |    25 | no                        | no                        |
| DK-FG-12      |    50 | yes                       | no                        |
| DK-HG-12      |    50 | yes                       | no                        |
| DK-VG-09      |    60 | yes                       | no                        |
| ES-SS-08      |    35 | yes                       | no                        |
| FI-MP-06      |    35 | no                        | no                        |
| FI-PB-98      |    40 | no                        | no                        |
| FI-WP-06      |    35 | no                        | no                        |
| GR-H1-97      |    35 | no                        | no                        |
| GR-P3-10      |    35 | no                        | no                        |
| GR-PA-08      |    35 | no                        | no                        |
| IT-I4-96      |    36 | no                        | no                        |
| KS-PR-11      |    62 | no                        | no                        |
| NL-KP-03      |    38 | yes                       | no                        |
| NL-KP-05      |    37 | yes                       | no                        |
| NL-KP-09      |    38 | yes                       | no                        |
| UK-SP-06      |    25 | yes                       | no                        |
| US-WS-09      |   100 | yes                       | no                        |
| ZA-LW-09      |   148 | no                        | no                        |
| ZA-WD-09      |   42  | no                        | no                        |
|               |       |                           |                           |
| **XHSTT2014A**|       |                           |                           |
| Abramson15    |    30 | no                        | no                        |
| All11         |   121 | no                        | no                        |
| All15         |   225 | no                        | no                        |
| hdtt4         |    30 | no                        | no                        |
| hdtt5         |    30 | no                        | no                        |
| hdtt6         |    30 | no                        | no                        |
| hdtt7         |    30 | no                        | no                        |
| hdtt8         |    30 | no                        | no                        |
| Sudoku4x4     |     4 | no                        | no                        |























