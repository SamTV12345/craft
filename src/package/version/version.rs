use regex::Regex;

use crate::package::version::connector::Connector;

use super::{constraint::VersionConstraint, contracts::Version, field::VersionField, group::VersionGroup, operator::Operator};

#[derive(Debug, Clone)]
pub struct VersionImpl {
    inner: Vec<VersionGroup>,
}

// ─── ToString ────────────────────────────────────────────────────────────────

impl ToString for VersionImpl {
  fn to_string(&self) -> String {
      self.inner
          .iter()
          .map(|constraint| constraint.to_string())
          .collect::<Vec<_>>()
          .join(" ")
  }
}

// ─── Impl ────────────────────────────────────────────────────────────────────

impl VersionImpl {
  fn parse_constraints(version: &str) -> Vec<VersionGroup> {
      if version.contains("||") {
          let parts = version.split("||").collect::<Vec<_>>();
          let mut groups = vec![];

          for part in parts {
              let mut constraints = vec![];
              if part.contains(">") || part.contains("<") {
                  constraints.append(&mut Self::parse_range(&part));
              } else {
                  constraints.push(VersionConstraint::parse(part));
              }

              let group = VersionGroup::new(constraints, Connector::Or);
              groups.push(group);
          }

          return groups;
      }

      if version.contains(">") || version.contains("<") {
          let constraints = Self::parse_range(&version);

          return vec![VersionGroup::new(constraints, Connector::And)];
      }

      let constraint = VersionConstraint::parse(version);

      vec![VersionGroup::new(vec![constraint], Connector::And)]
  }

  fn parse_range(version: &str) -> Vec<VersionConstraint> {
      let regex = r"^(?P<start_operator>[<>]=?|~|\^)?(?P<start_major>\d+|x|\*)(?:\.(?P<start_minor>\d+|x|\*))?(?:\.(?P<start_patch>\d+|x|\*))?(?:(?P<connector>,|\|\|)?\s*(?P<end_operator>[<>]=?|~|\^)?(?P<end_major>\d+|x|\*)(?:\.(?P<end_minor>\d+|x|\*))?(?:\.(?P<end_patch>\d+|x|\*))?)?$";

      let mut start_operator = Operator::Equal;
      let mut start_major = VersionField::Wildcard;
      let mut start_minor = VersionField::Wildcard;
      let mut start_patch = VersionField::Wildcard;

      let mut end_operator = Operator::Equal;
      let mut end_major = VersionField::Wildcard;
      let mut end_minor = VersionField::Wildcard;
      let mut end_patch = VersionField::Wildcard;

      let regex = Regex::new(regex).unwrap();

      let captures = regex
          .captures(&version)
          .expect(format!("Invalid version: {}", version).as_str());

      if let Some(start_operator_value) = captures.name("start_operator") {
          start_operator = start_operator_value.as_str().parse::<Operator>().unwrap();
      }

      if let Some(start_major_value) = captures.name("start_major") {
          if start_major_value.as_str() != "*" && start_major_value.as_str() != "x" {
              start_major =
                  VersionField::Exact(start_major_value.as_str().parse::<u64>().unwrap());
          }
      }

      if let Some(start_minor_value) = captures.name("start_minor") {
          if start_minor_value.as_str() != "*" && start_minor_value.as_str() != "x" {
              start_minor =
                  VersionField::Exact(start_minor_value.as_str().parse::<u64>().unwrap());
          }
      }

      if let Some(start_patch_value) = captures.name("start_patch") {
          if start_patch_value.as_str() != "*" && start_patch_value.as_str() != "x" {
              start_patch =
                  VersionField::Exact(start_patch_value.as_str().parse::<u64>().unwrap());
          }
      }

      if let Some(end_operator_value) = captures.name("end_operator") {
          end_operator = end_operator_value.as_str().parse::<Operator>().unwrap();
      }

      if let Some(end_major_value) = captures.name("end_major") {
          if end_major_value.as_str() != "*" && end_major_value.as_str() != "x" {
              end_major = VersionField::Exact(end_major_value.as_str().parse::<u64>().unwrap());
          }
      }

      if let Some(end_minor_value) = captures.name("end_minor") {
          if end_minor_value.as_str() != "*" && end_minor_value.as_str() != "x" {
              end_minor = VersionField::Exact(end_minor_value.as_str().parse::<u64>().unwrap());
          }
      }

      if let Some(end_patch_value) = captures.name("end_patch") {
          if end_patch_value.as_str() != "*" && end_patch_value.as_str() != "x" {
              end_patch = VersionField::Exact(end_patch_value.as_str().parse::<u64>().unwrap());
          }
      }

      let mut constraints = vec![];

      // Check if there was any start version specified

      if start_major != VersionField::Wildcard
          || start_minor != VersionField::Wildcard
          || start_patch != VersionField::Wildcard
      {
          constraints.push(VersionConstraint {
              operator: start_operator,
              major: start_major,
              minor: start_minor,
              patch: start_patch,
              pre_release: None,
              build: None,
          });
      }

      // Check if there was any end version specified

      if end_major != VersionField::Wildcard
          || end_minor != VersionField::Wildcard
          || end_patch != VersionField::Wildcard
      {
          constraints.push(VersionConstraint {
              operator: end_operator,
              major: end_major,
              minor: end_minor,
              patch: end_patch,
              pre_release: None,
              build: None,
          });
      }

      constraints
  }
}


// ─── Version ───────────────────────────────────────────────────────────────


impl Version for VersionImpl {
  fn new(version: &str) -> Self {
      let inner = Self::parse_constraints(&version);

      Self { inner }
  }

  fn is_exact(&self) -> bool {
      if self.inner.len() != 1 {
          return false;
      }

      let group = &self.inner[0];

      if group.constraints.len() != 1 {
          return false;
      }

      let constraint = &group.constraints[0];

      if constraint.operator != Operator::Equal {
          return false;
      }

      match (&constraint.major, &constraint.minor, &constraint.patch) {
          (VersionField::Exact(_), VersionField::Exact(_), VersionField::Exact(_)) => true,
          _ => false,
      }
  }

  fn satisfies(&self, version: &str) -> bool {
      todo!()
  }
}
